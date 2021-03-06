extern crate bindgen;


use std::env;
use std::env::var;
use std::path::{Path, PathBuf};
use std::fs;
use std::mem::transmute;

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
    cfg.profile("Release");
    cfg.define("CMAKE_INSTALL_PREFIX", out_path);

    cfg.define("ENABLE_CL", "OFF");

    let enable_cl = env::var_os("CARGO_FEATURE_CL").is_some();
    if enable_cl{
        println!("cargo:waning=enable cl" );
        cfg.define("ENABLE_CL", "ON");
    }


    let target_os = var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os.contains("linux")  {
        cfg.generator("Ninja");
    }

    if target_os == "android" {
        let toolchain = var("CARGO_NDK_CMAKE_TOOLCHAIN_PATH").unwrap();
        let android_platform = var("CARGO_NDK_ANDROID_PLATFORM").unwrap();
        let android_abi = var("CARGO_NDK_ANDROID_TARGET").unwrap();
        cfg.generator("Ninja");
        cfg.define("CMAKE_TOOLCHAIN_FILE", toolchain);
        cfg.define("ANDROID_PLATFORM", android_platform);
        cfg.define("ANDROID_ABI", android_abi);
        // cfg.define("ANDROID_STL", "c++_static");
    }
    cfg.define("BUILD_TESTS", "OFF");
    let dst = cfg.build();

    let out_build_dir = dst.parent().unwrap().parent().unwrap().parent().unwrap();
    let deps = out_build_dir.join("deps");

    let dyn_dir = dst.join("bin");
    let lib_dir = dst.join("lib");
    println!("cargo:rustc-link-search={}", dyn_dir.display());
    println!("cargo:rustc-link-search={}", lib_dir.display());

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