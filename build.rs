extern crate bindgen;

use std::env::{self};
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let root_dir = PathBuf::from(
        &std::env::var("CARGO_MANIFEST_DIR").expect("Should have CARGO_MANIFEST_DIR"),
    );
    let quickjs_dir = root_dir.join("quickjs");

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=libregexp");

    // Rebuild if any of these files change
    [
        quickjs_dir.join("libregexp.c"),
        quickjs_dir.join("libunicode.c"),
        quickjs_dir.join("cutils.c"),
        root_dir.join("shims.c"),
        root_dir.join("build.rs"),
    ]
    .into_iter()
    .for_each(|p| {
        assert!(p.exists(), "File does not exist: {}", p.to_str().unwrap());
        println!("cargo:rerun-if-changed={}", p.to_str().unwrap());
    });

    cc::Build::new()
        .file(quickjs_dir.join("libregexp.c"))
        .file(quickjs_dir.join("libunicode.c"))
        .file(quickjs_dir.join("cutils.c"))
        .file(root_dir.join("shims.c"))
        .warnings(false)
        .opt_level(2)
        .compile("libregexp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(
            quickjs_dir
                .join("libregexp.h")
                .to_str()
                .unwrap()
                .to_string(),
        )
        .header(
            quickjs_dir
                .join("libunicode.h")
                .to_str()
                .unwrap()
                .to_string(),
        )
        .header(quickjs_dir.join("cutils.h").to_str().unwrap().to_string())
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
