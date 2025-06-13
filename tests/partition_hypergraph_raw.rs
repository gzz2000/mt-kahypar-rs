//! From mt-kahypar-sc/lib/examples/partition_hypergraph.cc

use std::{
    ffi::{CStr, CString},
    ptr,
};

use mt_kahypar::sys as ffi;

const EXPECT_IMBALANCE: f64 = 0.023682559598494413;
const EXPECT_KM1: i32 = 224;
const EXPECT_BLOCK_WEIGHT_0: i32 = 6225;
const EXPECT_BLOCK_WEIGHT_1: i32 = 6527;

#[test]
fn deterministic_partitioning() {
    unsafe {
        let mut error = ffi::mt_kahypar_error_t {
            msg: ptr::null(),
            msg_len: 0,
            status: ffi::mt_kahypar_status_t::SUCCESS,
        };

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        ffi::mt_kahypar_initialize(num_threads, true);

        let context =
            ffi::mt_kahypar_context_from_preset(ffi::mt_kahypar_preset_type_t::DETERMINISTIC);
        assert!(!context.is_null(), "context == NULL");

        ffi::mt_kahypar_set_partitioning_parameters(
            context,
            2,
            0.03,
            ffi::mt_kahypar_objective_t::KM1,
        );
        ffi::mt_kahypar_set_seed(42);

        let verbose_val = CString::new("0").unwrap();
        let status = ffi::mt_kahypar_set_context_parameter(
            context,
            ffi::mt_kahypar_context_parameter_type_t::VERBOSE,
            verbose_val.as_ptr(),
            &mut error,
        );
        assert_eq!(status, ffi::mt_kahypar_status_t::SUCCESS);

        let file = CString::new("tests/ibm01.hgr").unwrap();
        let hypergraph = ffi::mt_kahypar_read_hypergraph_from_file(
            file.as_ptr(),
            context,
            ffi::mt_kahypar_file_format_type_t::HMETIS,
            &mut error,
        );
        if hypergraph.hypergraph.is_null() {
            panic_with_error("Load hypergraph failed", &mut error);
        }

        let partitioned_hg = ffi::mt_kahypar_partition(hypergraph, context, &mut error);
        if partitioned_hg.partitioned_hg.is_null() {
            panic_with_error("Partition failed", &mut error);
        }

        let mut block_weights = [0 as ffi::mt_kahypar_hypernode_weight_t; 2];
        ffi::mt_kahypar_get_block_weights(partitioned_hg, block_weights.as_mut_ptr());

        let imbalance = ffi::mt_kahypar_imbalance(partitioned_hg, context);
        let km1 = ffi::mt_kahypar_km1(partitioned_hg);

        assert!(
            (imbalance - EXPECT_IMBALANCE).abs() < 1e-12,
            "imbalance {} != expected {}",
            imbalance,
            EXPECT_IMBALANCE
        );
        assert_eq!(km1, EXPECT_KM1, "km1 mismatch");
        assert_eq!(
            block_weights[0], EXPECT_BLOCK_WEIGHT_0,
            "block 0 weight mismatch"
        );
        assert_eq!(
            block_weights[1], EXPECT_BLOCK_WEIGHT_1,
            "block 1 weight mismatch"
        );

        ffi::mt_kahypar_free_context(context);
        ffi::mt_kahypar_free_hypergraph(hypergraph);
        ffi::mt_kahypar_free_partitioned_hypergraph(partitioned_hg);
    }
}

unsafe fn panic_with_error(prefix: &str, error: &mut ffi::mt_kahypar_error_t) -> ! {
    let msg = if !error.msg.is_null() {
        CStr::from_ptr(error.msg).to_string_lossy().into_owned()
    } else {
        "<no error message>".into()
    };
    ffi::mt_kahypar_free_error_content(error);
    panic!("{}: {}", prefix, msg);
}
