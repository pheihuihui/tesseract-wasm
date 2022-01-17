fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    // println!("cargo:rustc-link-lib=tesseract");
}
