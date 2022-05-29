extern crate bindgen;


use std::env;
use std::path::PathBuf;

fn main(){
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let header =  root
        .join("easyFFT").join("src").join("include").join("easyFFT.h");
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=easyFFT");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", header.display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header.display().to_string())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(root.join("src").join("bindings.rs").display().to_string())
        .expect("Couldn't write bindings!");

    let vars = env::vars();
    for one in vars {
        println!("cargo:warning={}:{}", one.0, one.1);
    }

    use cmake::Config;

    let dst = Config::new("easyFFT")
        .build();

    println!("cargo:warning=CMAKE_DST:{}", dst.display());

    println!("cargo:rustc-link-search=native={}", dst.join("bin").display());
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=easyFFT");


}