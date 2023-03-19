fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,/opt/nvidia/deepstream/deepstream-6.1/lib/");
}
