use std::ffi::CString;
use std::os::raw::c_char;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn loaded_extension();
    }
}

fn loaded_extension() {
    println!("Sqlite Blob Extension loaded!");
}

#[no_mangle]
pub extern "C" fn sqlite_blob_duckdb_extension_init() {}

#[no_mangle]
pub extern "C" fn sqlite_blob_duckdb_extension_version() -> *const c_char {
    let c_string = CString::new("v0.5.1").expect("This won't fail.");
    let ptr = c_string.as_ptr();
    std::mem::forget(c_string);
    ptr
}

fn main() {}
