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
    #[error("[E008] The query contains unknown placeholder - query: `{0}`")]
    UnknownPlaceholder(String),
}

/*
#[derive(Debug)]
pub enum TsGeneratorError {
    EmptyQueryNameFromAnnotation(String),
    EmptyQueryNameFromVarDecl(String),
    MissingAliasForFunctions(String),
    InvalidTypescriptFilePath(PathBuf),
    // Wildcard expr handler errors
    WildcardStatementWithoutTargetTables,
    WildcardStatementDeadendExpression,
    WildcardStatementUnsupportedTableExpr,
    // Expression errors
    UnknownPlaceholder(String),
}

impl fmt::Display for TsGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQueryNameFromAnnotation(query) => writeln!(
                f,
                "Failed to fetch query name from DB name annotation - query: {}",
                query,
            ),
            Self::EmptyQueryNameFromVarDecl(query) => {
                writeln!(
                    f,
                    "[E001] Unable to infer an appropriate name for the query - query {:#?}",
                    query
                )
            }
            Self::MissingAliasForFunctions(query) => {
                writeln!(f, "Missing alias when handling functions - query: {}", query,)
            }
            Self::InvalidTypescriptFilePath(path_buf) => {
                writeln!(f, "Invalid Typescript file path - file path: {:?}", path_buf,)
            }
            // Wildcard expr handling errors
            Self::WildcardStatementWithoutTargetTables => {
                writeln!(
                    f,
                    "Failed to handle a wildcard statement without target tables in `FROM` statement"
                )
            }
            Self::WildcardStatementDeadendExpression => {
                writeln!(
                    f,
                    "Failed to handle a wildcard statement as it reached a dead-end expression"
                )
            }
            Self::WildcardStatementUnsupportedTableExpr => {
                writeln!(f, "Unsupported table with joins statement detected")
            }
            // Expression errors
            Self::UnknownPlaceholder(placeholder) => {
                writeln!(f, "The query contains unknown placeholder {}", placeholder)
            }
        }
    }
}
 */
