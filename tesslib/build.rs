fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    // println!("cargo:rustc-link-search=native=../sys-build/usr");
    // println!("cargo:rustc-link-lib=tesseract");
}
