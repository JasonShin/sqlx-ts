use std::fmt;
use std::path::PathBuf;

// todo: change it to ts generator error
#[derive(Debug)]
pub enum TsGeneratorError {
    EmptyQueryNameFromAnnotation(String),
    EmptyQueryNameFromVarDecl,
    MissingAliasForFunctions(String),
    InvalidTypescriptFilePath(PathBuf),
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
            ),
            Self::InvalidTypescriptFilePath(path_buf) => writeln!(
                f,
                "Invalid Typescript file path - file path: {:?}",
                path_buf,
            ),
        }
    }
}
