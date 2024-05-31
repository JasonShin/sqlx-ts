use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsGeneratorError {
    #[error("[E001] Unable to infer an appropriate name for the query - query: `{0}`")]
    EmptyQueryNameFromVarDecl(String),
    #[error("[E002] Failed to fetch query name from the annotation - query: `{0}`")]
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
    #[error("[E008] The query contains unknown placeholder parameter symbol - query: `{0}`")]
    UnknownPlaceholder(String),
    #[error("[E009] When translating a function in a SELECT clause, you must provide an alias - query : `{0}`")]
    FunctionWithoutAliasInSelectClause(String),
    #[error("[E010] Unknown function detected while processing a SELECT clause - query: `{0}`")]
    FunctionUnknown(String),
    #[error("[E011] table name was not found while processing an identifier - query: `{0}`")]
    IdentifierWithoutTable(String),
    #[error("[E012] Failed to handle a from statement without the `FROM` keyword - query: `{0}`")]
    FromWithoutKeyword(String),
    #[error("Unknown error: `{0}`")]
    Unknown(String),
}
