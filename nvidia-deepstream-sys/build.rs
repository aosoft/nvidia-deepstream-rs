extern crate bindgen;

use std::env;
use std::path::PathBuf;

static NVDS_VER: &str = "6.1";

fn main() {
    let pk = pkg_config::Config::new().probe("gstreamer-1.0").unwrap();

    println!("cargo:rustc-link-search=/opt/nvidia/deepstream/deepstream-{}/lib/", NVDS_VER);
    for path in &pk.link_paths {
        println!("cargo:rustc-link-search={:?}", path);
    }
    println!("cargo:rustc-link-lib=dylib=nvdsgst_meta");
    println!("cargo:rustc-link-lib=dylib=nvds_meta");
    println!("cargo:rustc-link-lib=dylib=nvds_yml_parser");
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I/opt/nvidia/deepstream/deepstream-{}/sources/includes/", NVDS_VER));
    for path in &pk.include_paths {
        bindings = bindings.clang_arg(format!("-I{}", path.to_str().unwrap()).as_str());
    }

    bindings = bindings.parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings = bindings
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
