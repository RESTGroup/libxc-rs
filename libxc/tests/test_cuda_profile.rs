//! Profiling test for CUDA compute performance (100K grid points).
//!
//! Run: LIBXC_DYLOAD=/home/a/Software/libxc-devel/lib/libxc.so \
//!   cargo test -p libxc --features cuda --test test_cuda_profile --
//!   --nocapture

#[cfg(feature = "cuda")]
#[cfg(feature = "api-v7_1")]
mod profile {
    use libxc::compute_cuda::*;
    use libxc::enums::libxc_enum_items::*;
    use libxc::prelude::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Instant;

    const NPOINTS: usize = 100_000;

    fn host_to_device(
        stream: &Arc<cudarc::driver::CudaStream>,
        data: &[f64],
    ) -> cudarc::driver::CudaSlice<f64> {
        let mut slice = unsafe { stream.alloc(data.len()).expect("CUDA alloc failed") };
        stream.memcpy_htod(data, &mut slice).expect("CUDA memcpy failed");
        slice
    }

    fn device_to_host(
        stream: &Arc<cudarc::driver::CudaStream>,
        slice: &cudarc::driver::CudaSlice<f64>,
    ) -> Vec<f64> {
        let mut host = vec![0.0f64; slice.len()];
        stream.memcpy_dtoh(slice, &mut host).expect("CUDA memcpy back failed");
        host
    }

    #[test]
    fn profile_gga_cuda_compute() {
        let n_warmup = 3;
        let n_iter = 10;

        let rho_host: Vec<f64> = (0..NPOINTS).map(|i| (i as f64) * 0.01).collect();
        let sigma_host: Vec<f64> = (0..NPOINTS).map(|i| (i as f64) * 0.001).collect();

        // ========== Infrastructure phases ==========

        // --- Phase 0: Dynamic library loading (first FFI call triggers it) ---
        let t0 = Instant::now();
        let _n_func = libxc::util::libxc_number_of_functionals();
        let dt_dyload = t0.elapsed();
        eprintln!(
            "Phase 0: Dynamic library loading        : {:>8.3} ms",
            dt_dyload.as_secs_f64() * 1e3
        );

        // --- Phase 1: CUDA context + stream ---
        let t1 = Instant::now();
        let stream = {
            let ctx = cudarc::driver::CudaContext::new(0).expect("Failed to create CUDA context");
            ctx.default_stream()
        };
        stream.synchronize().expect("sync failed");
        let dt_cuda_ctx = t1.elapsed();
        eprintln!(
            "Phase 1: CUDA context + stream          : {:>8.3} ms",
            dt_cuda_ctx.as_secs_f64() * 1e3
        );

        // ========== Functional creation ==========

        // --- Phase 2: Functional creation (device) ---
        let t2 = Instant::now();
        let func_gpu = LibXCFunctional::from_identifier_with_device(
            "gga_c_pbe",
            Unpolarized,
            LibXCDeviceFlag::OnDevice,
        );
        let dt_func_create_gpu = t2.elapsed();
        eprintln!(
            "Phase 2: Functional creation (GPU)      : {:>8.3} ms",
            dt_func_create_gpu.as_secs_f64() * 1e3
        );

        // --- Phase 3: Functional creation (CPU) ---
        let t3 = Instant::now();
        let func_cpu = LibXCFunctional::from_identifier("gga_c_pbe", Unpolarized);
        let dt_func_create_cpu = t3.elapsed();
        eprintln!(
            "Phase 3: Functional creation (CPU)      : {:>8.3} ms",
            dt_func_create_cpu.as_secs_f64() * 1e3
        );

        // ========== GPU pipeline ==========

        // --- Phase 4: H2D data transfer ---
        let t4 = Instant::now();
        let rho_slice = host_to_device(&stream, &rho_host);
        let sigma_slice = host_to_device(&stream, &sigma_host);
        stream.synchronize().expect("sync failed");
        let dt_h2d = t4.elapsed();
        eprintln!(
            "Phase 4: H→D transfer ({NPOINTS} pts)      : {:>8.3} ms",
            dt_h2d.as_secs_f64() * 1e3
        );

        // --- Phase 5: GPU CUDA compute (warmup + timed) ---
        let mut input_gpu: LibXCCudaInput = HashMap::new();
        input_gpu.insert("rho".to_string(), rho_slice.as_view());
        input_gpu.insert("sigma".to_string(), sigma_slice.as_view());

        for _ in 0..n_warmup {
            let _ = func_gpu.cuda_compute_xc(&stream, &input_gpu, 1).unwrap();
            stream.synchronize().expect("sync failed");
        }

        let mut dt_gpu_sum = std::time::Duration::ZERO;
        for _ in 0..n_iter {
            let t = Instant::now();
            let _ = func_gpu.cuda_compute_xc(&stream, &input_gpu, 1).unwrap();
            stream.synchronize().expect("sync failed");
            dt_gpu_sum += t.elapsed();
        }
        let dt_gpu_avg = dt_gpu_sum / n_iter;
        eprintln!(
            "Phase 5: GPU compute {NPOINTS} pts (avg {n_iter}) : {:>8.3} ms",
            dt_gpu_avg.as_secs_f64() * 1e3
        );

        // --- Phase 6: D2H data transfer ---
        let (buffer, _) = func_gpu.cuda_compute_xc(&stream, &input_gpu, 1).unwrap();
        stream.synchronize().expect("sync failed");

        let t6 = Instant::now();
        let _result = device_to_host(&stream, &buffer);
        let dt_d2h = t6.elapsed();
        eprintln!(
            "Phase 6: D→H transfer ({NPOINTS} pts)      : {:>8.3} ms",
            dt_d2h.as_secs_f64() * 1e3
        );

        // ========== CPU pipeline ==========

        // --- Phase 7: CPU compute (warmup + timed) ---
        let input_cpu: HashMap<String, &[f64]> = HashMap::from([
            ("rho".to_string(), rho_host.as_slice()),
            ("sigma".to_string(), sigma_host.as_slice()),
        ]);

        for _ in 0..n_warmup {
            let _ = func_cpu.compute_xc(&input_cpu, 1).unwrap();
        }

        let mut dt_cpu_sum = std::time::Duration::ZERO;
        for _ in 0..n_iter {
            let t = Instant::now();
            let _ = func_cpu.compute_xc(&input_cpu, 1).unwrap();
            dt_cpu_sum += t.elapsed();
        }
        let dt_cpu_avg = dt_cpu_sum / n_iter;
        eprintln!(
            "Phase 7: CPU compute {NPOINTS} pts (avg {n_iter}) : {:>8.3} ms",
            dt_cpu_avg.as_secs_f64() * 1e3
        );

        // ========== Summary ==========

        let gpu_pipeline = dt_h2d + dt_gpu_avg + dt_d2h;
        eprintln!("---");
        eprintln!("Summary ({NPOINTS} grid points, gga_c_pbe, unpolarized):");
        eprintln!(
            "  Functional creation:  GPU {:>6.3} ms  |  CPU {:>6.3} ms",
            dt_func_create_gpu.as_secs_f64() * 1e3,
            dt_func_create_cpu.as_secs_f64() * 1e3,
        );
        eprintln!(
            "  Compute only:         GPU {:>6.3} ms  |  CPU {:>6.3} ms  |  speedup {:.1}x",
            dt_gpu_avg.as_secs_f64() * 1e3,
            dt_cpu_avg.as_secs_f64() * 1e3,
            dt_cpu_avg.as_secs_f64() / dt_gpu_avg.as_secs_f64(),
        );
        eprintln!(
            "  GPU full pipeline (H→D+compute+D→H): {:>6.3} ms  |  vs CPU: {:.1}x",
            gpu_pipeline.as_secs_f64() * 1e3,
            dt_cpu_avg.as_secs_f64() / gpu_pipeline.as_secs_f64(),
        );
        eprintln!(
            "  One-time costs:  dyload {:>6.3} ms  |  CUDA ctx {:>6.3} ms",
            dt_dyload.as_secs_f64() * 1e3,
            dt_cuda_ctx.as_secs_f64() * 1e3,
        );
    }
}

/* Test output example

Setup: AMD 9955HX + NVidia RTX 5070 Laptop (mechrevo jiaolong 16 pro)

Phase 0: Dynamic library loading        :    2.314 ms
Phase 1: CUDA context + stream          :  224.277 ms
Phase 2: Functional creation (GPU)      :    0.312 ms
Phase 3: Functional creation (CPU)      :    0.004 ms
Phase 4: H→D transfer (100000 pts)      :    2.973 ms
Phase 5: GPU compute 100000 pts (avg 10) :    0.692 ms
Phase 6: D→H transfer (100000 pts)      :    0.309 ms
Phase 7: CPU compute 100000 pts (avg 10) :    6.511 ms
---
Summary (100000 grid points, gga_c_pbe, unpolarized):
  Functional creation:  GPU  0.312 ms  |  CPU  0.004 ms
  Compute only:         GPU  0.692 ms  |  CPU  6.511 ms  |  speedup 9.4x
  GPU full pipeline (H→D+compute+D→H):  3.975 ms  |  vs CPU: 1.6x
  One-time costs:  dyload  2.314 ms  |  CUDA ctx 224.277 ms

*/
