use crate::{common::lazy::{CONFIG, DB_SCHEMA}, ts_generator::types::ts_query::TsFieldType};
use sqlparser::ast::{DataType};

pub fn translate_data_type(
    data_type: &DataType
) -> TsFieldType {
    match &data_type {
        DataType::Character(_) => TsFieldType::String,
        DataType::Char(_) => TsFieldType::String,
        DataType::CharacterVarying(_) => TsFieldType::String,
        DataType::CharVarying(_) => TsFieldType::String,
        DataType::Varchar(_) => TsFieldType::String,
        DataType::Nvarchar(_) => TsFieldType::String,
        DataType::Uuid => TsFieldType::String,
        DataType::CharacterLargeObject(_) => TsFieldType::String,
        DataType::CharLargeObject(_) => TsFieldType::String,
        DataType::Clob(_) => TsFieldType::String,
        DataType::Binary(_) => TsFieldType::String,
        DataType::Varbinary(_) => TsFieldType::String,
        DataType::Blob(_) => TsFieldType::String,
        DataType::Numeric(_) => TsFieldType::Number,
        DataType::Decimal(_) => TsFieldType::Number,
        DataType::BigNumeric(_) => TsFieldType::Number,
        DataType::BigDecimal(_) => TsFieldType::Number,
        DataType::Dec(_) => TsFieldType::Number,
        DataType::Float(_) => TsFieldType::Number,
        DataType::TinyInt(_) => TsFieldType::Number,
        DataType::UnsignedTinyInt(_) => TsFieldType::Number,
        DataType::SmallInt(_) => TsFieldType::Number,
        DataType::UnsignedSmallInt(_) => TsFieldType::Number,
        DataType::MediumInt(_) => TsFieldType::Number,
        DataType::UnsignedMediumInt(_) => TsFieldType::Number,
        DataType::Int(_) => TsFieldType::Number,
        DataType::Integer(_) => TsFieldType::Number,
        DataType::UnsignedInt(_) => TsFieldType::Number,
        DataType::UnsignedInteger(_) => TsFieldType::Number,
        DataType::BigInt(_) => TsFieldType::Number,
        DataType::UnsignedBigInt(_) => TsFieldType::Number,
        DataType::Real => TsFieldType::Number,
        DataType::Double => TsFieldType::Number,
        DataType::DoublePrecision => TsFieldType::Number,
        DataType::Boolean => TsFieldType::Boolean,
        DataType::Date => TsFieldType::Date,
        DataType::Time(_, _) => TsFieldType::Date,
        DataType::Datetime(_) => TsFieldType::Date,
        DataType::Timestamp(_, _) => TsFieldType::String,
        DataType::Interval => TsFieldType::Any,
        DataType::JSON => TsFieldType::Object,
        DataType::Regclass => TsFieldType::String,
        DataType::Text => TsFieldType::String,
        DataType::String => TsFieldType::String,
        DataType::Bytea => TsFieldType::String,
        DataType::Custom(_, _) => TsFieldType::Any,
        DataType::Array(_) => todo!(),
        DataType::Enum(_) => todo!(),
        DataType::Set(_) => todo!(),
    }
}
