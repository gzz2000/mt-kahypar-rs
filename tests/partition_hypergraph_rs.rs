//! From mt-kahypar-sc/lib/examples/partition_hypergraph.cc

use mt_kahypar::{
    Context, FileFormat, Hypergraph, Objective, Preset,
};

const EXPECT_IMBALANCE: f64 = 0.023682559598494413;
const EXPECT_KM1: i32 = 224;
const EXPECT_BLOCK_WEIGHT_0: i32 = 6225;
const EXPECT_BLOCK_WEIGHT_1: i32 = 6527;

#[test]
fn deterministic_partitioning_rust_api() -> mt_kahypar::Result<()> {
    // let n_threads = std::thread::available_parallelism()
    //     .map(|n| n.get())
    //     .unwrap_or(1);
    // initialize(n_threads, /* interleaved = */ true);

    let ctx = Context::builder()
        .preset(Preset::Deterministic)
        .k(2)
        .epsilon(0.03)
        .objective(Objective::Km1)
        .seed(42)
        .verbose(false)
        .build()?;

    let hg = Hypergraph::from_file("tests/ibm01.hgr", &ctx, FileFormat::HMetis)?;

    let phg = hg.partition()?;

    let imbalance = phg.imbalance();
    assert!(
        (imbalance - EXPECT_IMBALANCE).abs() < 1e-12,
        "imbalance {imbalance} ≠ expected {EXPECT_IMBALANCE}"
    );
    assert_eq!(phg.km1(), EXPECT_KM1, "km1 mismatch");

    let bw = phg.block_weights();
    assert_eq!(bw[0], EXPECT_BLOCK_WEIGHT_0, "block-0 weight mismatch");
    assert_eq!(bw[1], EXPECT_BLOCK_WEIGHT_1, "block-1 weight mismatch");

    Ok(())
}
