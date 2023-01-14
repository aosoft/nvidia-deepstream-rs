static NVDS_VER: &str = "6.1";

fn main() {
    println!(
        "cargo:rustc-link-search=/opt/nvidia/deepstream/deepstream-{}/lib/",
        NVDS_VER
    );
    println!("cargo:rustc-link-lib=dylib=nvdsgst_meta");
    println!("cargo:rustc-link-lib=dylib=nvds_meta");
    println!("cargo:rustc-link-lib=dylib=nvds_yml_parser");

}