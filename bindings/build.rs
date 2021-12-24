extern crate bindgen;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let builder_lept = bindgen::Builder::default().header("wrapper_leptonica.h");
    let builder_tess_capi = bindgen::Builder::default()
        .header("wrapper_tess_capi.h")
        .allowlist_function("^Tess.*")
        .blocklist_type("Boxa")
        .blocklist_type("Pix")
        .blocklist_type("Pixa")
        .blocklist_type("_IO_FILE")
        .blocklist_type("_IO_codecvt")
        .blocklist_type("_IO_marker")
        .blocklist_type("_IO_wide_data");
    let builder_tess_types = bindgen::Builder::default()
        .header("wrapper_tess_public_types.hpp")
        .allowlist_var("^k.*")
        .blocklist_item("kPolyBlockNames");

    let bindings_lept = builder_lept
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for lept");
    let bindings_tess_capi = builder_tess_capi
        .generate()
        .expect("Unable to generate bindings for tess");
    let bindings_tess_types = builder_tess_types
        .generate()
        .expect("Unable to generate bindings for tess types");

    let out_path = PathBuf::from("../tesswasm/src/generated");

    bindings_lept
        .write_to_file(out_path.join("bindings_leptonica.rs"))
        .expect("Couldn't write bindings for leptonica.");

    let path = out_path.join("bindings_tess_capi.rs");
    let capi_cont = bindings_tess_capi.to_string();
    let mut file1 = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    file1
        .write("use crate::generated::bindings_leptonica::{Boxa, Pix, Pixa, _IO_FILE};\n\n".as_bytes())
        .unwrap();
    let mut file2 = OpenOptions::new().append(true).open(&path).unwrap();
    file2.write(capi_cont.as_bytes()).unwrap();

    bindings_tess_types
        .write_to_file(out_path.join("bindings_tess_types.rs"))
        .expect("Couldn't write bindings for tess types.");
}
