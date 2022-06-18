use std::fmt;

// todo: change it to ts generator error
#[derive(Debug)]
pub enum TsGeneratorError {
    EmptyQueryNameFromAnnotation(String),
    EmptyQueryNameFromVarDecl,
}

impl fmt::Display for TsGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQueryNameFromAnnotation(query) => writeln!(
                f,
                "Failed to fetch query name from DB name annotation - query: {}",
                query
            ),
            Self::EmptyQueryNameFromVarDecl => todo!(),
        }
    }
}
