% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/relational.R
\name{rel_distinct}
\alias{rel_distinct}
\title{Lazily compute a distinct result on a DuckDB relation object}
\usage{
rel_distinct(rel)
}
\arguments{
\item{rel}{the input DuckDB relation object}
}
\value{
a new \code{duckdb_relation} object with distinct rows
}
\description{
Lazily compute a distinct result on a DuckDB relation object
}
\examples{
con <- DBI::dbConnect(duckdb())
rel <- rel_from_df(con, mtcars)
rel2 <- rel_distinct(rel)
}
