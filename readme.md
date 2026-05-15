# libxc FFI bindings and wrappers

This project contains Rust FFI bindings and a safe wrapper for [libxc](https://gitlab.com/libxc/libxc), the library of exchange-correlation functionals for density-functional theory (DFT).

This crate is not an official libxc project. It is originally intended to potentially serve the Rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest).

## Crate `libxc`

The user-facing safe wrapper around libxc. It owns the [`LibXCFunctional`] type, manages resource lifetimes, validates inputs, and provides idiomatic Rust APIs for constructing functionals, querying their properties, and computing exchange-correlation energies and derivatives on a grid.

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/libxc.svg)](https://crates.io/crates/libxc) |
| API Document | [![API Documentation](https://docs.rs/libxc/badge.svg)](https://docs.rs/libxc) |

### Dynamic loading

By default, the C library is loaded at runtime via `libloading`. Set the `LIBXC_DYLOAD` environment variable to the path of the libxc shared library, or let the crate search standard paths (`LD_LIBRARY_PATH`, conda environments, system dirs). Disable the `dynamic_loading` cargo feature to use static linking instead.

### Example: GGA energy and potential

```rust
use libxc::prelude::*;
use libxc_enum_items::*;
use std::collections::HashMap;

// Create a GGA correlation functional (unpolarized)
let func = LibXCFunctional::from_identifier("gga_c_pbe", Unpolarized);

// Prepare input: 3 grid points
let rho: Vec<f64>   = vec![0.1, 0.2, 0.3];
let sigma: Vec<f64> = vec![0.01, 0.02, 0.03];
let mut input = HashMap::new();
input.insert("rho".into(), rho.as_slice());
input.insert("sigma".into(), sigma.as_slice());

// Compute energy (zk) and first derivative (vrho, vsigma)
let (buf, layout) = func.compute_xc(&input, 1).unwrap();
let zk = &buf[layout.get("zk").unwrap()];
let vrho = &buf[layout.get("vrho").unwrap()];
println!("zk = {:?}", zk);
println!("vrho = {:?}", vrho);
```

### Cargo features of `libxc`

Default features:
- **`api-v7_0`**: Binds libxc v7.0.0 API (cumulative with `api-v6_2`).
- **`dynamic_loading`**: Loads libxc at runtime via `libloading`.

Other features:
- **`api-v6_2`**: Binds libxc v6.2.2 (base API).
- **`api-v7_1`**: Binds libxc v7.1.0 API (cumulative with `api-v7_0`). Adds `xc_func_init_flags` and runtime device selection. Note v7.1.0 is not a released version (devel branch of original libxc), API may change for v7.1.
- **`cuda`**: GPU computation support via `cudarc`.
  - Creates GPU functionals with [`LibXCFunctional::from_identifier_with_device`] and [`LibXCDeviceFlag::OnDevice`]; computes with [`cuda_compute_xc`] (and per-family `cuda_compute_lda`/`gga`/`mgga`, plus `_with_output`/`_with_unsliced_output` variants).
  - Input/output types are [`LibXCCudaInput`] (`HashMap<String, CudaView<f64>>`) and [`LibXCCudaOutputMut`] (`HashMap<String, CudaViewMut<f64>>`).
  - **v7.1 vs v7.0**: With v7.1, a single libxc shared library can run both CPU and CUDA functionals; device is selected per-functional via `xc_func_init_flags`. With v7.0, the shared library is compiled as either CPU-only or CUDA-only — device is a compile-time property of the library, not per-functional. Please call the correct compute function to the corresponding CPU/CUDA shared library.

## Crate `libxc-ffi`

Raw, unsafe FFI bindings (1:1 with the C `xc_*` API). Users should prefer the `libxc` wrapper. The FFI bindings are auto-generated from libxc headers by scripts in `libxc-ffi/scripts/`.

## Workspace structure

```text
libxc-rs/
├── libxc/          # Safe Rust wrapper (public API)
├── libxc-ffi/      # Raw FFI bindings (auto-generated)
│   ├── headers/    # Versioned copies of libxc C headers
│   └── scripts/    # Code generation scripts
```

This project is partially developed with Claude Code. Please refer to directory `.claude` and file `ARCHITECTURE.md` for developers.

## Testing

```bash
LIBXC_DYLOAD=/path/to/libxc.so cargo test -p libxc
```

## License

Apache-2.0. This project is a derivative work of [libxc](https://libxc.gitlab.io/) and contains headers and AI-translated/generated code from the original library (MPL-2.0).
