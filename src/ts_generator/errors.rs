use std::fmt;

// todo: change it to ts generator error
#[derive(Debug)]
pub enum TsGeneratorError {
    EmptyQueryNameFromAnnotation(String),
    EmptyQueryNameFromVarDecl,
    MissingAliasForFunctions(String),
}

impl fmt::Display for TsGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQueryNameFromAnnotation(query) => writeln!(
                f,
                "Failed to fetch query name from DB name annotation - query: {}",
                query,
            ),
            Self::EmptyQueryNameFromVarDecl => todo!(),
            Self::MissingAliasForFunctions(query) => writeln!(
                f,
                "Missing alias when handling functions - query: {}",
                query,
            )
        }
    }
}
