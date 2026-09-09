# Changelog

## v0.1.4 -- 2026-09-09

Enhancement:

- `LibXCFunctional` is now `Clone`: clones are reference-counted handles to
  the same underlying C functional (no FFI-level re-initialization), so one
  functional can be shared across rayon workers lock-free. The unsafe
  `Send`/`Sync` impls moved to the private owning handle; their soundness is
  argued from libxc's `const xc_func_type *` read-side API
  (`tests/general/test_clone.rs`).
- All mutating setters now additionally require `Arc` uniqueness: while any
  clone of the functional is alive they fail with the new
  `LibXCError::SharedError` (or panic for the non-`_f` variants). The
  threshold setters (`set_dens_threshold`, `set_zeta_threshold`,
  `set_sigma_threshold`, `set_tau_threshold`, `set_fhc_enforcement`) gained
  fallible `_f` variants accordingly.

Fix:

- Graceful handling of external-parameter value getters on libxc < 7.0: the
  `xc_func_get_ext_params*` getters were introduced in libxc v7.0, and 6.x
  has no uniform read-back of current values. Previously this surfaced as an
  opaque panic inside the dynamic-loading shim (or a segfault when a second
  libxc library was also discoverable, e.g. a conda environment). Now
  `ext_param_values_f`/`ext_param_map_f` return the new
  `LibXCError::UnsupportedVersion` on such libraries, `describe()` degrades
  to default values with a note, and `set_ext_param_map` seeds from defaults
  (matching libxc 6.x semantics where setting one parameter resets the
  others).

Change:

- `LibXCError` is now `#[non_exhaustive]`: new variants may be added in
  patch releases, so downstream matches should include a wildcard (`_`) arm.

## v0.1.3 -- 2026-09-08

Fix:

- libxc 6.2.x support
  - the workspace `libxc-ffi` dependency now sets `default-features = false`,
    so api-version features are selected exclusively through the `libxc`
    crate; previously `--no-default-features --features api-v6_2` still
    compiled the v7.0 struct layout of `xc_func_type` and read garbage
    against a v6.2.2 shared library
  - mGGA tau handling: `needs_tau()` falls back to the functional family for
    pre-v7.0 libraries; zeroed scratch substituted for null lapl/tau inputs
    and for tau-family outputs missing from the layout/output map (libxc
    6.2.x aborts the process otherwise)
  - CI: the v7.0 test workflow now selects `dynamic_loading` explicitly and
    pins conda-forge `libxc` to `7.0.*`; it previously relied on the implicit
    feature unification fixed above, and conda-forge's default libxc is now
    7.1.2

Enhancement:

- Refresh headers from released libxc tags: `xc_funcs_v7.1.h` and `xc.h`
  from 7.1.2 (the v7.1 line is now released upstream as 7.1.0–7.1.2; it was
  previously snapshotted from the unreleased `devel` branch), and
  `xc_funcs_v6.2.h` from 6.2.2. `XcFuncId` enums regenerated.
  - v7.1: add `T_HLE17`, `LDA_C_BJ89`, `GGA_X_LLP`, `LDA_C_LP96_B`,
    `LDA_K_LP96_B`, `LDA_C_RPAF`, `HYB_MGGA_XC_COACH`,
    `MGGA_X_SREGTM_V1/V2/V3`; remove `MS2BS`, `MVSB`, `MVSBS`, `OPB3LYP`
    (present only in the pre-release snapshot, never in a released v7.1)
  - v6.2: remove functional IDs that do not exist in libxc 6.2.2; fix
    `LDA_C_1D_CSS` -> `LDA_C_1D_CSC` and `LDA_XC_TH_FL` -> `GGA_XC_TH_FL`;
    add `MGGA_X_MK00`

## v0.1.2 -- 2026-05-25

Enhancement:

- Enhance to layout handling
  - use IndexMap instead of Vec for storing components
  - make `components` field public
  - add method `iter_to_range`

- Docsrs features updated

- Add Send and Sync to `LibXCFunctional`.

## v0.1.1 -- 2026-05-15

Enhancement:

- Added function `set_ext_param_by_name`.

## v0.1.0 -- 2026-05-15

First version.

Supports libxc v6.2 (cpu), v7.0 (cpu, cuda), devel-branch at this stage (cpu and cuda).
