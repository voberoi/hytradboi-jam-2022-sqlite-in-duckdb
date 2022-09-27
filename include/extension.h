#pragma once

#include "duckdb.hpp"

namespace duckdb {
    class SqliteBlobExtension : public Extension {
    public:
        void Load(DuckDB &db) override;
        std::string Name() override;
    };
}
