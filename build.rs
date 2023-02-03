extern crate bindgen;

use std::env::{self, args};
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let libdir_path = PathBuf::from(
        &std::env::var("CARGO_MANIFEST_DIR").expect("Should have CARGO_MANIFEST_DIR"),
    )
    .join("quickjs");

    let libregexp_header = libdir_path.join("libregexp.h");
    let libunicode_header = libdir_path.join("libunicode.h");
    let cutils_header = libdir_path.join("cutils.h");

    let cwd = PathBuf::from(
        &std::env::var("CARGO_MANIFEST_DIR").expect("Should have CARGO_MANIFEST_DIR"),
    );
    // This is the path to the intermediate object file for our library.
    let libregexp_obj_path = cwd.join("libregexp.o");
    let libunicode_obj_path = cwd.join("libregexp.o");
    let cutils_obj_path = cwd.join("libregexp.o");
    // This is the path to the static library file.
    let lib_path = libdir_path.join("libregexp.a");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=libregexp");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!(
        "cargo:rerun-if-changed={}",
        libregexp_header.to_str().unwrap()
    );
    println!(
        "cargo:rerun-if-changed={}",
        libunicode_header.to_str().unwrap()
    );
    println!("cargo:rerun-if-changed={}", cutils_header.to_str().unwrap());

    cc::Build::new()
    .file(libdir_path.join("libregexp.c"))
    .file(libdir_path.join("libunicode.c"))
    .file(libdir_path.join("cutils.c"))
    .define("TEST", "1")
    .compile("libregexp");


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(libregexp_header.to_str().unwrap().to_string())
        .header(libunicode_header.to_str().unwrap().to_string())
        .header(cutils_header.to_str().unwrap().to_string())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
