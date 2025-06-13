//! this build script compiles the mt-kahypar-sc into a static library

use std::env;
use std::time::SystemTime;
use glob::glob;

fn main() {
    println!("Building mt-kahypar-sc");
    println!("cargo:rerun-if-changed=mt-kahypar-sc");

    let mut builder_mtkahypar = cc::Build::new();
    builder_mtkahypar
        .cpp(true)
        .debug(false)
        .opt_level(3)
        .flag("-std=c++17")
        .flag("-Wno-unused-function")
        .include("mt-kahypar-sc/external_tools/boost_kahypar")
        .include("mt-kahypar-sc/external_tools/tbb_kahypar/oneTBB/include")
        .include("mt-kahypar-sc")
        .include("mt-kahypar-sc/external_tools/kahypar-shared-resources")
        .include("mt-kahypar-sc/external_tools/growt")
        .include("mt-kahypar-sc/external_tools/WHFC")
        .define("MT_KAHYPAR_LIBRARY_MODE", None)
        .define("KAHYPAR_DISABLE_HWLOC", None)
        .define("KAHYPAR_ENABLE_LARGE_K_PARTITIONING_FEATURES", None)
        .define("KAHYPAR_ENABLE_HIGHEST_QUALITY_FEATURES", None)
        .define("KAHYPAR_ENABLE_GRAPH_PARTITIONING_FEATURES", None)
        .define("KAHYPAR_ENABLE_SOED_METRIC", None)
        .define("KAHYPAR_ENABLE_STEINER_TREE_METRIC", None)
        .files(glob("mt-kahypar-sc/mt-kahypar/**/*.cpp").unwrap().filter_map(Result::ok))
        .file("mt-kahypar-sc/lib/mtkahypar.cpp");
    #[cfg(feature = "int64_ids")] {
        builder_mtkahypar.define("KAHYPAR_USE_64_BIT_IDS", None);
    }
    builder_mtkahypar.compile("mtkahyparsc");
    println!("cargo:rustc-link-lib=static=mtkahyparsc");

    let mut builder_tbbmalloc = cc::Build::new();
    builder_tbbmalloc
        .cpp(true)
        .debug(false)
        .opt_level(3)
        .flag("-std=c++17")
        .include("mt-kahypar-sc/external_tools/tbb_kahypar/oneTBB/include")
        .define("__TBBMALLOC_BUILD", None)
        .define("__TBB_DYNAMIC_LOAD_ENABLED", "0")
        .define("__TBB_SOURCE_DIRECTLY_INCLUDED", "1")
        .files(glob("mt-kahypar-sc/external_tools/tbb_kahypar/oneTBB/src/tbbmalloc/*.cpp").unwrap().filter_map(Result::ok))
        .file("mt-kahypar-sc/external_tools/tbb_kahypar/oneTBB/src/tbb/itt_notify.cpp")
        .compile("tbbmallockahypar");
    println!("cargo:rustc-link-lib=static=tbbmallockahypar");

    let mut builder_tbb = cc::Build::new();
    builder_tbb
        .cpp(true)
        .debug(false)
        .opt_level(3)
        .flag("-std=c++17")
        .include("mt-kahypar-sc/external_tools/tbb_kahypar/oneTBB/include")
        .define("__TBB_BUILD", None)
        .define("__TBB_DYNAMIC_LOAD_ENABLED", "0")
        .define("__TBB_SOURCE_DIRECTLY_INCLUDED", "1")
        .files(glob("mt-kahypar-sc/external_tools/tbb_kahypar/oneTBB/src/tbb/*.cpp").unwrap().filter_map(Result::ok))
        .compile("tbbkahypar");
    println!("cargo:rustc-link-lib=static=tbbkahypar");

    let mut builder_boost = cc::Build::new();
    builder_boost
        .cpp(true)
        .debug(false)
        .opt_level(3)
        .flag("-std=c++17")
        .include("mt-kahypar-sc/external_tools/boost_kahypar")
        .files(glob("mt-kahypar-sc/external_tools/boost_kahypar/libs/program_options/src/*.cpp").unwrap().filter_map(Result::ok))
        .compile("boostkahypar");
    println!("cargo:rustc-link-lib=static=boostkahypar");

    println!("cargo:mtkahypar_build_time={}",
             SystemTime::now()
             .duration_since(SystemTime::UNIX_EPOCH).unwrap()
             .as_nanos());
    println!("cargo:mtkahypar_manifest_dir={}",
             env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:mtkahypar_out_dir={}",
             env::var("OUT_DIR").unwrap());
}
