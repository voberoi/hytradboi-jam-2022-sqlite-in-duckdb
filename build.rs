fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/main.cc")
        .flag_if_supported("-std=c++11")
        .compile("bindings");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/main.cc");
    println!("cargo:rerun-if-changed=include/main.h");
}
