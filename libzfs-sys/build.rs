extern crate bindgen;

fn main() {
    let out_file = std::env::current_dir()
        .unwrap()
        .join("src")
        .join("libzfs-sys.rs");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .constified_enum_module("boolean")
        .clang_arg("-I/usr/include/libspl")
        .clang_arg("-I/usr/include/libzfs")
        .clang_arg("-I/usr/src/zfs-0.7.12/include")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to src.
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=zpool");
    println!("cargo:rustc-link-lib=zfs");
    println!("cargo:rustc-link-lib=nvpair");
    println!("cargo:rerun-if-changed=build.rs");
}
