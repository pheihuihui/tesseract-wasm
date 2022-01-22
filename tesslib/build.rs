fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rustc-link-search=static=../sys-build/usr/lib");
    // println!("cargo:rustc-link-lib=tesseract");
    // println!("cargo:rustc-link-lib=leptonica");
}
