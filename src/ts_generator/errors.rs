use std::fmt;
use std::path::PathBuf;

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
                writeln!(f, "[E001] Unable to infer an appropriate name for the query - query {:#?}", query)
            },
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
