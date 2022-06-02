extern crate bindgen;


use std::env;
use std::env::var;
use std::path::PathBuf;
use std::fs;

fn main(){
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let header =  root
        .join("easyFFT").join("src").join("include").join("easyFFT.h");

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



    use cmake::Config;

    let mut cfg = Config::new("easyFFT");

    let target_os = var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "android" {
        let toolchain = var("CARGO_NDK_CMAKE_TOOLCHAIN_PATH").unwrap();
        let android_platform = var("CARGO_NDK_ANDROID_PLATFORM").unwrap();
        let android_abi = var("CARGO_NDK_ANDROID_TARGET").unwrap();
        cfg.define("CMAKE_TOOLCHAIN_FILE", toolchain);
        cfg.generator("Ninja");
        cfg.define("ANDROID_PLATFORM", android_platform);
        cfg.define("ANDROID_ABI", android_abi);
        // cfg.define("ANDROID_STL", "c++_static");
    }

    let dst = cfg.build();


    let vars = env::vars();
    for one in vars {
        println!("cargo:warning={}:{}", one.0, one.1);
    }


    println!("cargo:rustc-link-search={}", dst.join("bin").display());
    println!("cargo:rustc-link-search={}", dst.join("lib").display());


    let out_build_dir = dst.parent().unwrap().parent().unwrap().parent().unwrap();
    let deps = out_build_dir.join("deps");
    println!("cargo:warning=DEPS_DIR:{}", deps.display());
    let dyn_dir = dst.join("bin");
    let lib_dir = dst.join("lib");


    if target_os == "windows" {
        let main_dll = dyn_dir.join("easyFFT.dll");
        copy_dyn(main_dll, deps.join("easyFFT.dll"));
        copy_dyn(dyn_dir.join("libfftw3-3.dll"), deps.join("libfftw3-3.dll"));
        copy_dyn(dyn_dir.join("libfftw3f-3.dll"), deps.join("libfftw3f-3.dll"));
        copy_dyn(dyn_dir.join("libfftw3l-3.dll"), deps.join("libfftw3l-3.dll"));
        println!("cargo:rustc-link-lib=easyFFT");
    }else {
        copy_dyn(lib_dir.join("libeasyFFT.so"), deps.join("libeasyFFT.so"));
        copy_dyn(lib_dir.join("libeasyFFT.so"), out_build_dir.join("libeasyFFT.so"));
        println!("cargo:rustc-link-lib=easyFFT");
    }
}

fn copy_dyn(src: PathBuf, dst: PathBuf){
    fs::copy(src, dst).unwrap();

}