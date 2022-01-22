fn main() {
    println!("cargo:rustc-link-search=../build/usr/lib");
    // println!("cargo:rustc-link-lib=static=tesseract");
    println!("cargo:rustc-link-lib=static=leptonica");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=png");
    println!("cargo:rustc-link-lib=jpeg");
    println!("cargo:rustc-link-lib=webp");
    println!("cargo:rustc-link-lib=gif");
    println!("cargo:rustc-link-lib=tiff");
    println!("cargo:rustc-link-lib=openjp2");
}
