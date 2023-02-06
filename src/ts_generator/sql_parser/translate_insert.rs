use sqlparser::ast::{Ident, Query};

use crate::ts_generator::{errors::TsGeneratorError, types::TsQuery};

pub fn translate_insert(ts_query: TsQuery, columns: &Vec<Ident>, source: &Box<Query>) -> Result<(), TsGeneratorError> {
    println!("translating columns and source {:?} - {:?}", columns, source);
    Ok(())
}
