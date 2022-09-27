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
// * Implement a DuckDB extension with a single function, "sqlite_blob_new()";
// * This should return a blob of binary bytes.
// * Implement sqlite_blob_query(str)
// * This should return a single number, 42.
// * This should work:
// *   - CREATE TABLE foo(id INTEGER, name VARCHAR, sqlite BLOB); // After step 1 also.
// *   - INSERT INTO foo VALUES (1, "vikram", sqlite_blob_new()), (2, "katie", sqlite_blob_new());
// *   - SELECT id, name, sqlite_query("<any str>") FROM foo;
