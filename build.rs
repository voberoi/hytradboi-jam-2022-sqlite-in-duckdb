fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/extension.cc")
        .flag_if_supported("-I./duckdb-0.5.1/src/include")
        .flag_if_supported("-std=c++11")
        .compile("sqlite_blob_extension_bindings");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/extension.cc");
    println!("cargo:rerun-if-changed=include/extension.h");
}
