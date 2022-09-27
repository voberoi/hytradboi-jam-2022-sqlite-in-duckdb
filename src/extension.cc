#define DUCKDB_BUILD_LOADABLE_EXTENSION

#include "duckdb.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"

#include "sqlite_blob_duckdb_extension/include/extension.h"
#include "sqlite_blob_duckdb_extension/src/main.rs.h"

namespace duckdb {
    void SqliteBlobExtension::Load(DuckDB &db) {
        loaded_extension();
    }

    std::string SqliteBlobExtension::Name() {
        return "sqlite_blob_duckdb_extension";
    }
}

extern "C" {
    DUCKDB_EXTENSION_API void sqlite_blob_duckdb_extension_init(duckdb::DatabaseInstance &db) {
        duckdb::DuckDB db_wrapper(db);
        db_wrapper.LoadExtension<duckdb::SqliteBlobExtension>();
    }

    DUCKDB_EXTENSION_API const char *sqlite_blob_duckdb_extension_version() {
        return duckdb::DuckDB::LibraryVersion();
    }
}

/*

 D LOAD './';
 Error: IO Error: Extension "./" not found

 D LOAD './target/release/hytradboi-jam-2022-sqlite-in-duckdb';
 Error: IO Error: File "./target/release/hytradboi-jam-2022-sqlite-in-duckdb" did not contain function "hytradboi-jam-2022-sqlite-in-duckdb_init": dlsym(0x203b4d7a0, hytradboi-jam-2022-sqlite-in-duckdb_init): symbol not found
 D
 */
