use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsGeneratorError {
    #[error("[E001] Unable to infer an appropriate name for the query - query: `{0}`")]
    EmptyQueryNameFromVarDecl(String),
    #[error("[E002] Failed to fetch query name from DB name annotation - query: `{0}`")]
    EmptyQueryNameFromAnnotation(String),
    #[error("[E003] Missing alias when handling functions - query: `{0}`")]
    MissingAliasForFunctions(String),
    #[error("[E004] Invalid Typescript file path - file path: `{0}`")]
    InvalidTypescriptFilePath(PathBuf),
    #[error("[E005] Failed to handle a wildcard statement without target tables in `FROM` statement - query: `{0}`")]
    WildcardStatementWithoutTargetTables(String),
    #[error("[E006] Failed to handle a wildcard statement as it reached a dead-end expression - query: `{0}`")]
    WildcardStatementDeadendExpression(String),
    #[error("[E007] Unsupported table with joins statement detected - query: `{0}`")]
    WildcardStatementUnsupportedTableExpr(String),
    #[error("[E008] The query contains unknown placeholder paramter symbol - query: `{0}`")]
    UnknownPlaceholder(String),
}
