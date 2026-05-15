---
name: cuda-testing
description: Instructions for testing CUDA compute in libxc-rs, including setup, running tests.
---

# CUDA Testing Instructions

This project supports CUDA compute via the `cuda` cargo feature, which requires
libxc v7.1+ compiled with CUDA support. Since CI environments typically lack
GPUs, CUDA testing is manual and local-only.

**Please note that you must pass the correct cargo feature to the corresponding version of the libxc shared library**:
- `api-v7_0` for libxc v7.0
- `api-v7_1` for libxc v7.1 (or the `devel` branch of libxc)

For cuda tests, you also need to pass the `cuda` feature (which implies `api-v7_1`).

**Known limitation for v7.0 + CUDA**: When libxc v7.0 is compiled with CUDA
(`--enable-cuda`), `libxc_malloc` uses `cudaMallocManaged` instead of `malloc`.
Our wrapper frees these strings with `libc::free`, which causes a crash. Two
tests (`test_xc_functional_get_name`, `test_xc_available_functional_names`) are
automatically `#[ignore]`d under `cfg(all(feature = "cuda", not(feature = "api-v7_1")))`.
This is a bug in the original libxc v7.0; v7.1+ is unaffected.

## Prerequisites

1. **GPU hardware** with CUDA toolkit installed (check: `nvidia-smi`)
2. **libxc shared library** compiled with CUDA support at a known path
3. **Conda environment** with cupy/numpy for pylibxc cross-validation

Set the following environment variables (or define them in `CLAUDE.local.md`):
- `LIBXC_DYLOAD_CUDA`: path to libxc shared library with CUDA support (v7.1+)
- `LIBXC_DYLOAD_CPU`: path to libxc shared library for CPU-only testing (v7.0)
- `PYLIBXC_CONDA_ENV`: conda environment name with pylibxc/cupy/numpy

## Build

```bash
LIBXC_DYLOAD=$LIBXC_DYLOAD_CUDA \
  cargo build -p libxc --features cuda
```

## Run CUDA Tests

```bash
LIBXC_DYLOAD=$LIBXC_DYLOAD_CUDA \
  cargo test -p libxc --features cuda --test test_cuda
```

Tests in `test_cuda.rs` are **not** `#[ignore]` — they run directly when the
test binary is executed. If no GPU is available, these tests will fail at CUDA
context creation.

## Run Non-CUDA Tests (Sanity Check)

Always verify the default (non-cuda) build still passes after changes:

```bash
LIBXC_DYLOAD=$LIBXC_DYLOAD_CPU \
  cargo test -p libxc
```

## Cross-Validation with pylibxc

To verify libxc's GPU path works and get reference values:

```bash
conda run -n $PYLIBXC_CONDA_ENV python -c "
import pylibxc, cupy as cp
xc = pylibxc.functional.LibXCFunctional('gga_c_pbe', 1,
    func_flags=pylibxc.flags.XC_FLAGS_ON_DEVICE)
inp = {'rho': cp.array([0., 1., 2., 3.]),
       'sigma': cp.array([0.0, 0.1, 0.2, 0.3])}
result = xc.compute(inp)
print('zk:', result['zk'].flatten())
print('vrho:', result['vrho'].flatten())
print('vsigma:', result['vsigma'].flatten())
"
```

## Key Architecture Notes

- The FFI calls (`xc_lda`, `xc_gga`, `xc_mgga`) are **identical** for CPU and
  GPU — libxc internally dispatches to CUDA kernels based on the functional's
  device flag set during `xc_func_init_flags`
- GPU functionals are created via `from_identifier_with_device(name, spin,
  LibXCDeviceFlag::OnDevice)` which calls `xc_func_init_flags`
- CPU functionals created via `from_identifier`/`from_number` use `init_func`
  which (with `api-v7_1`) calls `xc_func_init_flags` with `OnHost`
- The `compute_cuda` module takes `&Arc<CudaStream>` for all methods
- cudarc's `DevicePtr::device_ptr(&self, &stream)` returns `(CUdeviceptr,
  SyncOnDrop)` — the `SyncOnDrop` must stay alive until after the FFI call
- Output buffers use `alloc_zeros` (zero-initialized GPU memory) since libxc
  only writes non-null components
