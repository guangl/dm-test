use odbc_api::Error as OdbcError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("ODBC error: {0}")]
    Odbc(#[from] OdbcError),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Query error: {0}")]
    Query(String),
    #[error("Configuration error: {0}")]
    Config(String),
}
