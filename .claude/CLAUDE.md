# Agent instruction of Libxc Rust FFI and Wrapper

## Notes to human developers

You should also create a file `CLAUDE.local.md` to place local resources:
- `LIBXC_REPO_PATH`: local path of original libxc (c/cuda/fortran) repository. The source code can help you understand how libxc works.
- `DFTD4_DYLOAD`: the path of dftd4 dynamic library. During development, we usually use dynamic-loading. You need also to specify the version of the specified dftd4 dynamic library, and whether gpu is supported.

If human developers forgets to define `CLAUDE.local.md`, you should ask them to do so before you can start working on the FFI wrapper.

## Libxc original repository

General rules
- This repository should live at `LIBXC_REPO_PATH`, which is defined in `CLAUDE.local.md`. 
- **This repository should not be modified**, unless you are going to checkout specific tags (versions) of libxc.
- Main branch is `devel`, useful tags can be `6.2.2` and `7.0.0` (latest stable version at current time).

Important files for FFI and wrapper development:
- `src/*.h`: the headers. Note that these files are also copied to this project under `libxc-ffi/headers` folder.
- `pylibxc`: the python wrapper of libxc. We should at least implement all major features of the python wrapper, to translate the python wrapper to rust wrapper.
  - Note not all files are relevent to wrapper. The most relevent is `functional.py` and `util.py`. There are a lot of files relates to array-api and dlpack (generalize the numpy array to other tensor libraries), but that's not important to rust side.
- `pylibxc/example_densities.py`: this is not useful for wrapper, but useful for testing.
- `testsuite/regression`: this is useful for implementation correctness testing. However, these tests are too much, we may select some of them for testing, but not all of them.

## Build and test

We only test crate `libxc` during development.
Usually we use dynamic loading, and you need to set `DFTD4_DYLOAD` in environment variable. We may implement finding dftd4 dynamic library in the future, but environment variable `DFTD4_DYLOAD` can make sure we are using the exact version of dftd4 we want to test.

## General conventions

- We do not allow using external matrix/tensor libraries. However, for GPU support, we can use `cudarc` for type support of GPU tensors.
- We use row-major convention for tensor shapes, which is the same to original libxc python wrapper.
- Libxc may require multiple pointers (such as `"v3rho3", "v3rho2sigma", "v3rhosigma2", "v3sigma3"`). If possible, use tensor of `[4, ngrids]` instead of 4 separate vectors, which can minimize memory allocations.
- Please distinguish the use of word "flag". In original libxc, "flag" can be used as
  - Have exc/vxc/fxc/kxc/lxc (derivative level)
  - Requires rho/sigma/tau/grad (functional kind)
  - Device/host or gpu/cpu (device type)

## Handling of CPU and CUDA

We will currently separating CPU and CUDA implementation. CUDA will always have a prefix (general function `libxc_cuda_`, associated function `cuda_`), and CPU will not have a prefix (general function `libxc_`, associated function no prefix).

## Naming convention

- We will not use the original libxc prefix `xc_`.
- For functions and structs that will be exposed to users, add prefix `libxc_` for general functions, and `Libxc` for structs.
- If some function is to be fallible, we can add suffix `_f` (`fn <func>_f -> Result<_, LibxcError>`).

## Libxc and Header versioning

For this FFI wrapper, we will start from libxc v6.2.2.

At current time (2026-may), the latest stable version is v7.0.0, and developer version is v7.1.0. We will handle three versions: v6.2, v7.0, v7.1.
The cargo feature will activate the corresponding version of header (`api-v6_2`, `api-v7_0`, `api-v7_1`), and the default version is v7.0.
We will save libxc headers in folder `libxc-ffi/headers`.

- `xc.h` header is the main header for libxc. This header needs separate implementation of static FFI (`ffi_xc_static.rs`) and dynamic FFI (folder `ffi_xc_dynamic`). Different versions of libxc tags will be applied (such as `api_v6_2`, `api_v7_0`)
- `xc_funcs_<version>.h` contains the function declarations (const usize numbers) and minimal documentation for each functional. Different versions of libxc tags will be applied. This can contained at folder `xc_funcs`, defined as numbered enum (for future simplicity of parsing/serde).
- `versioning_xc.md` contains the detailed changes of libxc API for each version. This file is structured and maintained by human developer. Use `## Introduced in v<version>` to check what functions are introduced in each version (and can used for script to parse).

To generate the FFI bindings, use python scripts in `libxc-ffi/scripts`, with help of tree-sitter and bindgen. The FFI bindgen must be generated automatically, without manual modification.
