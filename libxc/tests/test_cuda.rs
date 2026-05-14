//! CUDA integration tests for libxc wrapper.
//!
//! These tests require a GPU and a libxc library compiled with CUDA support.
//! Run with: LIBXC_DYLOAD=/path/to/libxc.so cargo test -p libxc --features cuda
//! --test test_cuda

#[cfg(feature = "cuda")]
#[cfg(feature = "api-v7_1")]
mod cuda_tests {
    use libxc::compute_cuda::*;
    use libxc::enums::libxc_enum_items::*;
    use libxc::prelude::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Helper to create a CudaContext and default stream.
    fn get_cuda_stream() -> Arc<cudarc::driver::CudaStream> {
        let ctx = cudarc::driver::CudaContext::new(0).expect("Failed to create CUDA context");
        ctx.default_stream()
    }

    /// Copy host data to device.
    fn host_to_device(
        stream: &Arc<cudarc::driver::CudaStream>,
        data: &[f64],
    ) -> cudarc::driver::CudaSlice<f64> {
        let mut slice = unsafe { stream.alloc(data.len()).expect("CUDA alloc failed") };
        stream.memcpy_htod(data, &mut slice).expect("CUDA memcpy failed");
        slice
    }

    /// Copy device data back to host.
    fn device_to_host(
        stream: &Arc<cudarc::driver::CudaStream>,
        slice: &cudarc::driver::CudaSlice<f64>,
    ) -> Vec<f64> {
        let mut host = vec![0.0f64; slice.len()];
        stream.memcpy_dtoh(slice, &mut host).expect("CUDA memcpy back failed");
        host
    }

    #[test]
    fn test_gga_cuda_compute() {
        let stream = get_cuda_stream();

        // Create a GGA functional on device
        let func = LibXCFunctional::from_identifier_with_device(
            "gga_c_pbe",
            Unpolarized,
            LibXCDeviceFlag::OnDevice,
        );
        assert!(func.is_on_device());

        // Prepare input data (same as pylibxc test)
        let rho_host: Vec<f64> = vec![0., 1., 2., 3.];
        let sigma_host: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3];

        let rho_slice = host_to_device(&stream, &rho_host);
        let sigma_slice = host_to_device(&stream, &sigma_host);

        let mut input: LibXCCudaInput = HashMap::new();
        input.insert("rho".to_string(), rho_slice.as_view());
        input.insert("sigma".to_string(), sigma_slice.as_view());

        // Compute
        let (buffer, layout) = func.cuda_compute_xc(&stream, &input, 1).unwrap();

        // Copy results back
        let result = device_to_host(&stream, &buffer);

        // Validate against pylibxc reference values
        let zk_range = layout.get("zk").unwrap();
        let vrho_range = layout.get("vrho").unwrap();
        let vsigma_range = layout.get("vsigma").unwrap();

        let zk: Vec<f64> = result[zk_range.clone()].to_vec();
        let vrho: Vec<f64> = result[vrho_range.clone()].to_vec();
        let vsigma: Vec<f64> = result[vsigma_range.clone()].to_vec();

        // Reference values from pylibxc GPU test
        assert!((zk[1] - (-0.07077943)).abs() < 1e-6, "zk[1] = {}, expected -0.07077943", zk[1]);
        assert!(
            (vrho[1] - (-0.08001113)).abs() < 1e-6,
            "vrho[1] = {}, expected -0.08001113",
            vrho[1]
        );
        assert!(
            (vsigma[1] - 0.00417795).abs() < 1e-6,
            "vsigma[1] = {}, expected 0.00417795",
            vsigma[1]
        );
    }

    #[test]
    fn test_lda_cuda_compute() {
        let stream = get_cuda_stream();

        let func = LibXCFunctional::from_identifier_with_device(
            "lda_x",
            Unpolarized,
            LibXCDeviceFlag::OnDevice,
        );

        let rho_host: Vec<f64> = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let rho_slice = host_to_device(&stream, &rho_host);

        let mut input: LibXCCudaInput = HashMap::new();
        input.insert("rho".to_string(), rho_slice.as_view());

        let (buffer, layout) = func.cuda_compute_xc(&stream, &input, 1).unwrap();

        let result = device_to_host(&stream, &buffer);
        let zk_range = layout.get("zk").unwrap();
        let vrho_range = layout.get("vrho").unwrap();

        let zk: Vec<f64> = result[zk_range.clone()].to_vec();
        let vrho: Vec<f64> = result[vrho_range.clone()].to_vec();

        // CPU reference values for lda_x
        let func_cpu = LibXCFunctional::from_identifier("lda_x", Unpolarized);
        let cpu_input = HashMap::from([("rho".to_string(), rho_host.as_slice())]);
        let (cpu_buf, cpu_layout) = func_cpu.compute_xc(&cpu_input, 1).unwrap();
        let cpu_zk = &cpu_buf[cpu_layout.get("zk").unwrap()];
        let cpu_vrho = &cpu_buf[cpu_layout.get("vrho").unwrap()];

        for i in 0..zk.len() {
            assert!(
                (zk[i] - cpu_zk[i]).abs() < 1e-10,
                "zk[{i}] mismatch: GPU={}, CPU={}",
                zk[i],
                cpu_zk[i]
            );
            assert!((vrho[i] - cpu_vrho[i]).abs() < 1e-10, "vrho[{i}] mismatch",);
        }
    }

    #[test]
    fn test_cpu_guard_blocks_cuda_compute() {
        // Create a CPU functional (no device flag)
        let func = LibXCFunctional::from_identifier("gga_c_pbe", Unpolarized);
        assert!(!func.is_on_device());

        // Attempting CUDA compute on a CPU functional should error
        let stream = get_cuda_stream();
        let rho_host: Vec<f64> = vec![0.1, 0.2];
        let rho_slice = host_to_device(&stream, &rho_host);
        let mut input: LibXCCudaInput = HashMap::new();
        input.insert("rho".to_string(), rho_slice.as_view());
        let sigma_slice = host_to_device(&stream, &[0.01, 0.02]);
        input.insert("sigma".to_string(), sigma_slice.as_view());

        let result = func.cuda_compute_xc(&stream, &input, 1);
        assert!(result.is_err());
    }
}
