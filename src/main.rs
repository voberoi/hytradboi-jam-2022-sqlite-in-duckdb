// The plan: create a SQLite DuckDB column type and play around with it.
//
// - Store SQLite dbs in DuckDB blobs.
// - Provide functions to query these blobs.
// - Play around with a few use cases and analyze them.
// - Maybe provide other utility functions as I discove rthem.
//
// Currently need to:
//
// 1. Figure out how to write a simple DuckDB extension.
// 2. Figure out how to write this simple DuckDB extension in Rust.

// TODO:
// * Call into a Rust function in this crate from some C++ code.

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn a_rust_function();
    }

    unsafe extern "C++" {
        include!("hytradboi-jam-2022-sqlite-in-duckdb/include/main.h");
        fn a_cpp_function_that_calls_a_rust_function();
    }
}

fn a_rust_function() {
    println!("Hello from Rust!");
}

fn main() {
    ffi::a_cpp_function_that_calls_a_rust_function();
}
