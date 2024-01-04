use std::path::PathBuf;

fn main() {
    let include_path = PathBuf::from("C:/SLDevice/SDK/headers");
    let lib_path = "C:\\SLDevice\\SDK\\lib\\x64\\Release";

    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=SLImage");
    println!("cargo:rustc-link-lib=SLDeviceLib");
    println!("cargo:rustc-link-lib=libtiff");

    cxx_build::bridge("src/lib.rs")
    .file("src/test.cc")
    .include(include_path)
    .compile("cxx-demo");
}