use crate::ts_generator::types::ts_query::TsFieldType;
use sqlparser::ast::DataType;
use sqlparser::ast::Value;

pub fn translate_value(value: &Value) -> Option<TsFieldType> {
    match &value {
        Value::Number(_, _) => Some(TsFieldType::Number),
        Value::SingleQuotedString(_) => Some(TsFieldType::String),
        Value::DollarQuotedString(_) => Some(TsFieldType::String),
        Value::EscapedStringLiteral(_) => Some(TsFieldType::String),
        Value::SingleQuotedByteStringLiteral(_) => Some(TsFieldType::String),
        Value::DoubleQuotedByteStringLiteral(_) => Some(TsFieldType::String),
        Value::RawStringLiteral(_) => Some(TsFieldType::String),
        Value::NationalStringLiteral(_) => Some(TsFieldType::String),
        Value::HexStringLiteral(_) => Some(TsFieldType::String),
        Value::DoubleQuotedString(_) => Some(TsFieldType::String),
        Value::Boolean(_) => Some(TsFieldType::Boolean),
        Value::Null => Some(TsFieldType::Null),
        Value::UnQuotedString(_) => Some(TsFieldType::String),
        Value::Placeholder(_) => None,
    }
}

pub fn translate_data_type(data_type: &DataType) -> TsFieldType {
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
        DataType::JSONB => TsFieldType::Object,
        DataType::Regclass => TsFieldType::String,
        DataType::Text => TsFieldType::String,
        DataType::String(_) => TsFieldType::String,
        DataType::Bytea => TsFieldType::String,
        DataType::Custom(_, _) => TsFieldType::Any,
        DataType::Array(array_element_type_def) => match array_element_type_def {
            sqlparser::ast::ArrayElemTypeDef::None => TsFieldType::Array(Box::new(TsFieldType::Any)),
            sqlparser::ast::ArrayElemTypeDef::AngleBracket(data_type) => {
                TsFieldType::Array(Box::new(translate_data_type(data_type)))
            }
            sqlparser::ast::ArrayElemTypeDef::SquareBracket(data_type) => {
                TsFieldType::Array(Box::new(translate_data_type(data_type)))
            }
        },
        DataType::Enum(_) => TsFieldType::Array(Box::new(TsFieldType::String)),
        DataType::Set(_) => TsFieldType::Array(Box::new(TsFieldType::String)),
        DataType::Bytes(_) => TsFieldType::String,
        DataType::Int2(_) => TsFieldType::Number,
        DataType::UnsignedInt2(_) => TsFieldType::Number,
        DataType::Int4(_) => TsFieldType::Number,
        DataType::Int64 => TsFieldType::Number,
        DataType::UnsignedInt4(_) => TsFieldType::Number,
        DataType::Int8(_) => TsFieldType::Number,
        DataType::UnsignedInt8(_) => TsFieldType::Number,
        DataType::Float4 => TsFieldType::Number,
        DataType::Float64 => TsFieldType::Number,
        DataType::Float8 => TsFieldType::Number,
        DataType::Bool => TsFieldType::Boolean,
        DataType::Struct(_) => TsFieldType::Object,
        DataType::Unspecified => TsFieldType::Any,
    }
}
