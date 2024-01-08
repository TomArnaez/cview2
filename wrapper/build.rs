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
    println!("cargo:rustc-link-lib=libtiff");

    cxx_build::bridge("src/lib.rs")
    .file("src/test.cc")
    .includes(include_paths)
    .compile("cxx-demo");
}