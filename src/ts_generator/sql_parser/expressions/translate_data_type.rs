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
    Value::SingleQuotedRawStringLiteral(_) => Some(TsFieldType::String),
    Value::DoubleQuotedRawStringLiteral(_) => Some(TsFieldType::String),
    Value::TripleSingleQuotedString(_) => Some(TsFieldType::String),
    Value::TripleDoubleQuotedString(_) => Some(TsFieldType::String),
    Value::UnicodeStringLiteral(_) => Some(TsFieldType::String),
    Value::TripleSingleQuotedRawStringLiteral(_) => Some(TsFieldType::String),
    Value::TripleDoubleQuotedRawStringLiteral(_) => Some(TsFieldType::String),
    Value::TripleSingleQuotedByteStringLiteral(_) => Some(TsFieldType::String),
    Value::TripleDoubleQuotedByteStringLiteral(_) => Some(TsFieldType::String),
    Value::NationalStringLiteral(_) => Some(TsFieldType::String),
    Value::HexStringLiteral(_) => Some(TsFieldType::String),
    Value::DoubleQuotedString(_) => Some(TsFieldType::String),
    Value::Boolean(_) => Some(TsFieldType::Boolean),
    Value::Null => Some(TsFieldType::Null),
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
    DataType::DecimalUnsigned(_) => TsFieldType::Number,
    DataType::BigNumeric(_) => TsFieldType::Number,
    DataType::BigDecimal(_) => TsFieldType::Number,
    DataType::Dec(_) => TsFieldType::Number,
    DataType::DecUnsigned(_) => TsFieldType::Number,
    DataType::Float(_) => TsFieldType::Number,
    DataType::FloatUnsigned(_) => TsFieldType::Number,
    DataType::TinyInt(_) => TsFieldType::Number,
    DataType::TinyIntUnsigned(_) => TsFieldType::Number,
    DataType::SmallInt(_) => TsFieldType::Number,
    DataType::SmallIntUnsigned(_) => TsFieldType::Number,
    DataType::MediumInt(_) => TsFieldType::Number,
    DataType::MediumIntUnsigned(_) => TsFieldType::Number,
    DataType::Int(_) => TsFieldType::Number,
    DataType::IntUnsigned(_) => TsFieldType::Number,
    DataType::Integer(_) => TsFieldType::Number,
    DataType::IntegerUnsigned(_) => TsFieldType::Number,
    DataType::BigInt(_) => TsFieldType::Number,
    DataType::BigIntUnsigned(_) => TsFieldType::Number,
    DataType::Real => TsFieldType::Number,
    DataType::RealUnsigned => TsFieldType::Number,
    DataType::Double(_) => TsFieldType::Number,
    DataType::DoubleUnsigned(_) => TsFieldType::Number,
    DataType::DoublePrecision => TsFieldType::Number,
    DataType::DoublePrecisionUnsigned => TsFieldType::Number,
    DataType::Unsigned => TsFieldType::Number,
    DataType::UnsignedInteger => TsFieldType::Number,
    DataType::Boolean => TsFieldType::Boolean,
    DataType::Date => TsFieldType::Date,
    DataType::Time(_, _) => TsFieldType::Date,
    DataType::Datetime(_) => TsFieldType::Date,
    DataType::Timestamp(_, _) => TsFieldType::String,
    DataType::Interval { .. } => TsFieldType::Any,
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
      sqlparser::ast::ArrayElemTypeDef::SquareBracket(data_type, _) => {
        TsFieldType::Array(Box::new(translate_data_type(data_type)))
      }
      sqlparser::ast::ArrayElemTypeDef::Parenthesis(data_type) => {
        TsFieldType::Array(Box::new(translate_data_type(data_type)))
      }
    },
    DataType::Enum(_, _) => TsFieldType::Array(Box::new(TsFieldType::String)),
    DataType::Set(_) => TsFieldType::Array(Box::new(TsFieldType::String)),
    DataType::Bytes(_) => TsFieldType::String,
    DataType::Int2(_) => TsFieldType::Number,
    DataType::Int2Unsigned(_) => TsFieldType::Number,
    DataType::Int4(_) => TsFieldType::Number,
    DataType::Int4Unsigned(_) => TsFieldType::Number,
    DataType::Int64 => TsFieldType::Number,
    DataType::Int8(_) => TsFieldType::Number,
    DataType::Int8Unsigned(_) => TsFieldType::Number,
    DataType::Float4 => TsFieldType::Number,
    DataType::Float64 => TsFieldType::Number,
    DataType::Float8 => TsFieldType::Number,
    DataType::Bool => TsFieldType::Boolean,
    DataType::Struct(_, _) => TsFieldType::Object,
    DataType::Unspecified => TsFieldType::Any,
    // Handle all other variants with Any
    _ => TsFieldType::Any,
  }
}
