extern crate bindgen;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let out_path = PathBuf::from("../tesslib/src/generated");
    bind_lept(&out_path);
    bind_tess_capi(&out_path);
    bind_tess_types(&out_path);
}

fn bind_lept(out_path: &PathBuf) {
    let builder_lept = bindgen::Builder::default().header("headers/wrapper_leptonica.h");
    let bindings_lept = builder_lept
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for lept");
    let lept_content = bindings_lept.to_string();
    let from = "extern \"C\" {\n";
    let to = "#[link(name = \"leptonica\")]\nextern \"C\" {\n";
    let lept_content = lept_content.replace(from, to);
    let path = out_path.join("bindings_leptonica.rs");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    file.write(lept_content.as_bytes()).unwrap();
}

fn bind_tess_capi(out_path: &PathBuf) {
    let builder_tess_capi = bindgen::Builder::default()
        .header("headers/wrapper_tess_capi.h")
        .allowlist_function("^Tess.*")
        .blocklist_type("Boxa")
        .blocklist_type("Pix")
        .blocklist_type("Pixa")
        .blocklist_type("_IO_FILE")
        .blocklist_type("_IO_codecvt")
        .blocklist_type("_IO_marker")
        .blocklist_type("_IO_wide_data");
    let bindings_tess_capi = builder_tess_capi
        .generate()
        .expect("Unable to generate bindings for tess");
    let path = out_path.join("bindings_tess_capi.rs");
    let capi_content = bindings_tess_capi.to_string();
    let from = "extern \"C\" {\n";
    let to = "#[link(name = \"tesseract\")]\nextern \"C\" {\n";
    let capi_content = capi_content.replace(from, to);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    let txt = "use crate::generated::bindings_leptonica::{Boxa, Pix, Pixa, _IO_FILE};\n\n";
    file.write(txt.as_bytes()).unwrap();
    file.write(capi_content.as_bytes()).unwrap();
}

fn bind_tess_types(out_path: &PathBuf) {
    let builder_tess_types = bindgen::Builder::default()
        .header("headers/wrapper_tess_public_types.hpp")
        .allowlist_var("^k.*")
        .blocklist_item("kPolyBlockNames");
    let bindings_tess_types = builder_tess_types
        .generate()
        .expect("Unable to generate bindings for tess types");
    bindings_tess_types
        .write_to_file(out_path.join("bindings_tess_types.rs"))
        .expect("Couldn't write bindings for tess types.");
}
