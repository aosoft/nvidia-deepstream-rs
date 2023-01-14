static NVDS_VER: &str = "6.1";

fn main() {
    println!(
        "cargo:rustc-link-search=/opt/nvidia/deepstream/deepstream-{}/lib/",
        NVDS_VER
    );

    if cfg!(feature = "meta") {
        println!("cargo:rustc-link-lib=dylib=nvdsgst_meta");
        println!("cargo:rustc-link-lib=dylib=nvds_meta");
    }

    if cfg!(feature = "surface") {
        println!("cargo:rustc-link-lib=dylib=nvbufsurface");
    }

    if cfg!(feature = "yaml") {
        println!("cargo:rustc-link-lib=dylib=nvds_yml_parser");
    }
}
