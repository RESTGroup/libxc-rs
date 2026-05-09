# libxc-rs: Project Architecture

Rust FFI bindings and safe wrapper for [libxc](https://www.tddft.org/programs/libxc/), the library of exchange-correlation functionals for density-functional theory (DFT).

This document is generated with great help from AI code agent, minorly modified by human developer.

## Workspace Structure

```
libxc-rs/                         # Cargo workspace root
├── Cargo.toml                    # Workspace definition, shared package metadata
├── libxc-ffi/                    # Crate: raw FFI bindings (unsafe, 1:1 with C API)
│   ├── Cargo.toml
│   ├── headers/                  # Libxc C headers (versioned copies)
│   │   ├── xc.h                  # Main header (functions, structs, constants)
│   │   ├── xc_version.h          # Version constants (excluded from bindings)
│   │   ├── xc_funcs_vx.x.h       # Functional ID #defines for vx.x (v6_2, v7_0, v7_1, etc.)
│   │   └── versioning_xc.md      # Human-maintained API diff between versions
│   ├── scripts/                  # Code generation scripts
│   │   ├── generate_ffi.py       # Generates ffi_static.rs + ffi_dynamic/ from xc.h
│   │   └── generate_xc_funcs.py  # Generates xc_funcs/ enums from xc_funcs_v*.h
│   └── src/
│       ├── lib.rs                # Re-exports `ffi` as either static or dynamic
│       ├── ffi_static.rs         # Static linking FFI (bindgen + version gating, auto-generated)
│       ├── ffi_dynamic/          # Dynamic loading FFI module (auto-generated except mod.rs)
│       │   ├── mod.rs            # Dyload orchestration, library search logic
│       │   ├── ffi_base.rs       # Auto-generated: types, structs, constants
│       │   ├── dyload_struct.rs  # Auto-generated: DyLoadLib with Option<fn> fields
│       │   ├── dyload_initializer.rs  # Auto-generated: DyLoadLib::new symbol loading
│       │   └── dyload_compatible.rs   # Auto-generated: wrapper fns calling through dyload_lib()
│       └── xc_funcs/             # Functional ID enums (versioned, auto-generated)
│           ├── mod.rs            # Version-gated re-export of XcFuncId
│           └── vx_x.rs           # XcFuncId enum for vx.x (v6_2, v7_0, v7_1, etc.)
│
├── libxc/                        # Crate: safe Rust wrapper (public API for users)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                # Module declarations, prelude
│       ├── enums.rs              # Rust enums: Spin, Family, Kind, Flags, FuncId
│       ├── error.rs              # LibXCError enum
│       ├── functional.rs         # LibXCFunctional: creation, info, params, thresholds
│       ├── compute_cpu.rs        # LibXCFunctional: CPU compute methods (LDA/GGA/MGGA)
│       ├── layout_handling.rs    # LibXCOutputLayout, LibXCDerivativeFlags, output labels
│       └── util.rs               # Utility: version, functional lookup, library path
│
└── libxc/tests/                  # Integration tests
    └── <TODO for later>
```

## Crate Relationship

```
libxc (user-facing wrapper)
  └── libxc-ffi (raw FFI bindings)
        └── libxc C library (external, linked or loaded at runtime)
```

- **libxc-ffi**: Pure FFI layer. Translates the C `xc_*` API 1:1 to Rust `unsafe` functions. No safe abstractions. Users should not use this directly.
- **libxc**: Safe wrapper. Owns the `LibXCFunctional` type, manages resource lifetime (Drop), validates inputs, and provides idiomatic Rust APIs.

## FFI Binding Architecture

### Static vs Dynamic Loading

The `dynamic_loading` cargo feature (default: on) controls how the C library is accessed:

| Mode | Feature | Mechanism | FFI module |
|------|---------|-----------|------------|
| Dynamic loading | `dynamic_loading` (default) | `libloading` at runtime | `ffi_dynamic` |
| Static linking | no feature | Compile-time linking | `ffi_static` |

In `lib.rs`, the `ffi` alias is set:
```rust
#[cfg(not(feature = "dynamic_loading"))]
pub use ffi_static as ffi;
#[cfg(feature = "dynamic_loading")]
pub use ffi_dynamic as ffi;
```

### Dynamic Loading Flow

1. `dyload_lib()` (OnceLock) searches for the shared library across multiple candidate paths
2. `DyLoadLib::new()` resolves all `xc_*` symbols into `Option<extern "C" fn>` fields
3. `dyload_compatible.rs` wrapper functions call through `dyload_lib().<symbol>.unwrap()`
4. The wrapper signatures match `ffi_static.rs` exactly, so the `libxc` wrapper crate is agnostic

### Library Search Order (dynamic loading)

1. `LIBXC_DYLOAD` / `LIBXC_DYLOAD_XC` environment variable
2. `LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH` / `PATH` entries
3. Python interpreter discovery (`LIBXC_PYTHON_PATH`, `CONDA_PREFIX`, `PATH` python/python3)
4. Standard system paths (`/usr/lib`, `/usr/local/lib`, `/lib`)

## API Versioning

Three libxc versions are supported via cumulative cargo features:

| Feature | Libxc version | Adds |
|---------|--------------|------|
| `api-v6_2` | 6.2.2 | Base API |
| `api-v7_0` (default) | 7.0.0 | `xc_func_set_fhc_enforcement`, `xc_func_get_ext_params*`, `xc_mgga_new`, `ext_params` field |
| `api-v7_1` | 7.1.0 | `xc_func_info_get/set_default_flags`, `xc_func_init_flags` |

Features are cumulative: `api-v7_1` implies `api-v7_0` implies `api-v6_2`.

Version gating is applied at two levels:
- **Static FFI** (`ffi_static.rs`): `#[cfg(feature = "api-v7_0")]` on functions and struct fields
- **Dynamic FFI**: All symbols loaded at runtime; version features ignored (runtime panic if symbol missing)

## Code Generation

**All FFI binding files in `ffi_dynamic/` (except `mod.rs`) and `ffi_static.rs` are auto-generated. Do not edit them manually.**

### Generating FFI bindings

```bash
cd libxc-ffi/scripts
pip install tree-sitter tree-sitter-rust  # dependencies
python generate_ffi.py
```

This runs `bindgen` on `headers/xc.h`, then:
1. Generates `ffi_static.rs` with version-gated `#[cfg]` attributes
2. Generates `ffi_dynamic/ffi_base.rs`, `dyload_struct.rs`, `dyload_initializer.rs`, `dyload_compatible.rs`
3. Runs `cargo fmt`

### Generating functional ID enums

```bash
cd libxc-ffi/scripts
python generate_xc_funcs.py
```

Parses `headers/xc_funcs_v*.h` into Rust enums in `src/xc_funcs/`.

### Adding a new API version

1. Add the new header files to `libxc-ffi/headers/`
2. Update `versioning_xc.md` with new functions and struct changes
3. Add the version to `api_versions` and `version_new_functions` in `generate_ffi.py`
4. Add the version to `versions` in `generate_xc_funcs.py`
5. Add the cargo feature in both `Cargo.toml` files
6. Re-run both generation scripts

## Wrapper Crate Design (libxc)

### Core Type: `LibXCFunctional`

The central type, wrapping a `*mut ffi::xc_func_type` pointer. It is split across multiple files using separate `impl` blocks:

| File | Responsibility |
|------|---------------|
| `functional.rs` | Construction (`from_identifier`, `from_number`), info getters (`number`, `kind`, `family`, `flags`, `spin`, `dim`), references, description, external parameters, thresholds, hybrid/CAM/VV10 coefficients, auxiliary functionals, Drop |
| `compute_cpu.rs` | CPU compute methods: `compute_lda`, `compute_gga`, `compute_mgga`, and unified `compute_xc` dispatch |
| `layout_handling.rs` | `LibXCOutputLayout`, `LibXCDerivativeFlags`, output label tables, `validate_flags` |

### Compute API

Three compute modes per family (LDA/GGA/MGGA):

| Method | Output | Description |
|--------|--------|-------------|
| `compute_<family>` | `(Vec<f64>, Layout)` | Auto-allocates contiguous buffer |
| `compute_<family>_with_unsliced_output` | `Layout` | User provides flat `&mut [f64]` buffer |
| `compute_<family>_with_output` | `()` | User provides named `HashMap<&str, &mut [f64]>` |

Unified dispatch via `compute_xc` / `compute_xc_with_unsliced_output` / `compute_xc_with_output` routes to the correct family automatically.

### Input/Output Convention

- **Input**: `LibXCCpuInput = HashMap<&'static str, &[f64]>` with keys `"rho"`, `"sigma"`, `"lapl"`, `"tau"`
- **Output (sliced)**: `LibXCCpuOutputMut = HashMap<&'static str, &mut [f64]>` with keys like `"zk"`, `"vrho"`, etc.
- **Output (unsliced)**: Contiguous `&mut [f64]` interpreted via `LibXCOutputLayout`
- **Row-major**: Shapes are `[n_comp, npoints]`, last dimension contiguous (same as pylibxc)
- **Derivative levels**: Controlled by `LibXCDerivativeFlags` or `usize` (0=EXC, 1=+VXC, 2=+FXC, 3=+KXC, 4=+LXC)

### Output Layout

`LibXCOutputLayout` describes how a contiguous buffer maps to named components. Each component (e.g. `"zk"`, `"vrho"`) occupies `[offset .. offset + size)` within the buffer. Output label tables define the canonical ordering for LDA (5 components), GGA (15 components), and MGGA (up to 70 components).

### Enum Types

| Type | Description |
|------|-------------|
| `LibXCSpin` | Unpolarized (1) / Polarized (2) |
| `LibXCRelavistic` | Non-relativistic / Relativistic |
| `LibXCFunctionalKind` | Exchange / Correlation / Exchange-Correlation / Kinetic |
| `LibXCFamily` | LDA / GGA / MGGA / Hyb* / LCA / OEP |
| `LibXCFlags` | Bitflags for capabilities (HaveEXC/VXC/FXC/KXC/LXC, dimensions, CAM, VV10, etc.) |
| `LibXCFuncId` | Re-export of `XcFuncId` from `libxc-ffi`, identifies specific functionals |

### Error Handling

`LibXCError` has three variants:
- `NotFound(String)` -- functional or parameter not found
- `InitError { func_id, spin }` -- failed to initialize functional
- `ComputeError(String)` -- invalid inputs, family mismatch, etc.

Fallible methods use `_f` suffix (e.g. `from_identifier_f`, `from_number_f`); infallible wrappers panic on error.

## Naming Conventions

- No `xc_` prefix (original libxc) or `Xc` prefix
- General functions: `libxc_` prefix (e.g. `libxc_version`, `libxc_functional_get_number`)
- Structs: `LibXC` prefix (e.g. `LibXCFunctional`, `LibXCError`)
- Fallible variants: `_f` suffix
- CPU has no prefix; CUDA uses `libxc_cuda_` / `cuda_` prefix

## Testing

```bash
LIBXC_DYLOAD=/path/to/libxc.so cargo test -p libxc
```

The `LIBXC_DYLOAD` environment variable is required to point to the libxc shared library.

## Key Design Decisions

1. **No external tensor/matrix libraries** -- raw `&[f64]` slices only (exception: `cudarc` for GPU type support)
2. **FFI bindgen is auto-generated** -- manual edits only in `ffi_dynamic/mod.rs`
3. **Dynamic loading is the default** -- avoids compile-time libxc dependency
4. **CPU and CUDA are separated** -- CUDA always has explicit prefix (TODO: CUDA support is planned but not yet implemented, so this is future-proofing)
5. **Output uses contiguous buffers** -- allow multiple related pointers (e.g. `v3rho3`, `v3rho2sigma`, ...) packed into `[n_comp, npoints]` memory buffer to minimize allocations
