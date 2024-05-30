//
// Copyright (c) 2024 Nathan Fiedler
//
extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;
use std::process::Command;

const MIN_VERSION: &str = "0.6.24";
const MAX_VERSION: &str = "0.7";

fn main() {
    // for now, rely on pkg-config for finding the libexif paths
    let libexif = run_pkg_config();
    for include in libexif.include_paths.iter() {
        println!("cargo:include={}", include.display());
    }
    for path in libexif.link_paths.iter() {
        println!("cargo:rustc-link-search={}", path.display());
    }
    for lib in libexif.libs.iter() {
        // assume dynamic library for now
        println!("cargo:rustc-link-lib={}={}", "dylib", lib);
    }

    // invoke rust-bindgen to generate the libexif bindings dynamically
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for include in libexif.include_paths {
        builder = builder.clang_arg(format!("-I{}", include.display()));
    }
    let bindings = builder.generate().expect("could not generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("could not write bindings file");
}

fn run_pkg_config() -> pkg_config::Library {
    // assert that the appropriate version of libexif is installed
    pkg_config::Config::new()
        .cargo_metadata(false)
        .atleast_version(MIN_VERSION)
        .probe("libexif")
        .unwrap();
    if !Command::new("pkg-config")
        .arg(format!("--max-version={}", MAX_VERSION))
        .arg("libexif")
        .status()
        .unwrap()
        .success()
    {
        panic!("libexif version must be less than {}", MAX_VERSION);
    }
    pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libexif")
        .unwrap()
}
