use std::env;

fn main() {
    let include_env = env::var("SL_INCLUDE").unwrap();
    let include_paths: Vec<&str> = include_env.split(";").collect();
    let lib_env = env::var("SL_LIBS").unwrap();
    let lib_paths: Vec<&str> = lib_env.split(";").collect();

    for lib_path in lib_paths {
        println!("cargo:rustc-link-search=native={}", lib_path);
    }    

    println!("cargo:rustc-link-lib=SLImage");
    println!("cargo:rustc-link-lib=SLDeviceLib");

    cxx_build::bridge("src/lib.rs")
    .includes(include_paths)
    .flag_if_supported("-std=c++20")  // Specify C++20 standard
    .compile("cxx-demo");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.h");
}