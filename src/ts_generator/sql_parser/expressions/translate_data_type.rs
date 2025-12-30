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
    // Table types
    DataType::Table(_) => TsFieldType::Object,
    DataType::NamedTable { .. } => TsFieldType::Object,

    // String types
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
    DataType::Text => TsFieldType::String,
    DataType::TinyText => TsFieldType::String,
    DataType::MediumText => TsFieldType::String,
    DataType::LongText => TsFieldType::String,
    DataType::String(_) => TsFieldType::String,
    DataType::FixedString(_) => TsFieldType::String,

    // Binary types
    DataType::Binary(_) => TsFieldType::String,
    DataType::Varbinary(_) => TsFieldType::String,
    DataType::Blob(_) => TsFieldType::String,
    DataType::TinyBlob => TsFieldType::String,
    DataType::MediumBlob => TsFieldType::String,
    DataType::LongBlob => TsFieldType::String,
    DataType::Bytes(_) => TsFieldType::String,
    DataType::Bytea => TsFieldType::String,
    DataType::Bit(_) => TsFieldType::String,
    DataType::BitVarying(_) => TsFieldType::String,
    DataType::VarBit(_) => TsFieldType::String,

    // Numeric types
    DataType::Numeric(_) => TsFieldType::Number,
    DataType::Decimal(_) => TsFieldType::Number,
    DataType::DecimalUnsigned(_) => TsFieldType::Number,
    DataType::BigNumeric(_) => TsFieldType::Number,
    DataType::BigDecimal(_) => TsFieldType::Number,
    DataType::Dec(_) => TsFieldType::Number,
    DataType::DecUnsigned(_) => TsFieldType::Number,
    DataType::Float(_) => TsFieldType::Number,
    DataType::FloatUnsigned(_) => TsFieldType::Number,
    DataType::Float4 => TsFieldType::Number,
    DataType::Float32 => TsFieldType::Number,
    DataType::Float64 => TsFieldType::Number,
    DataType::Float8 => TsFieldType::Number,
    DataType::Real => TsFieldType::Number,
    DataType::RealUnsigned => TsFieldType::Number,
    DataType::Double(_) => TsFieldType::Number,
    DataType::DoubleUnsigned(_) => TsFieldType::Number,
    DataType::DoublePrecision => TsFieldType::Number,
    DataType::DoublePrecisionUnsigned => TsFieldType::Number,

    // Integer types
    DataType::TinyInt(_) => TsFieldType::Number,
    DataType::TinyIntUnsigned(_) => TsFieldType::Number,
    DataType::UTinyInt => TsFieldType::Number,
    DataType::SmallInt(_) => TsFieldType::Number,
    DataType::SmallIntUnsigned(_) => TsFieldType::Number,
    DataType::USmallInt => TsFieldType::Number,
    DataType::MediumInt(_) => TsFieldType::Number,
    DataType::MediumIntUnsigned(_) => TsFieldType::Number,
    DataType::Int(_) => TsFieldType::Number,
    DataType::IntUnsigned(_) => TsFieldType::Number,
    DataType::Int2(_) => TsFieldType::Number,
    DataType::Int2Unsigned(_) => TsFieldType::Number,
    DataType::Int4(_) => TsFieldType::Number,
    DataType::Int4Unsigned(_) => TsFieldType::Number,
    DataType::Int8(_) => TsFieldType::Number,
    DataType::Int8Unsigned(_) => TsFieldType::Number,
    DataType::Int16 => TsFieldType::Number,
    DataType::Int32 => TsFieldType::Number,
    DataType::Int64 => TsFieldType::Number,
    DataType::Int128 => TsFieldType::Number,
    DataType::Int256 => TsFieldType::Number,
    DataType::Integer(_) => TsFieldType::Number,
    DataType::IntegerUnsigned(_) => TsFieldType::Number,
    DataType::BigInt(_) => TsFieldType::Number,
    DataType::BigIntUnsigned(_) => TsFieldType::Number,
    DataType::HugeInt => TsFieldType::Number,
    DataType::UHugeInt => TsFieldType::Number,
    DataType::UBigInt => TsFieldType::Number,
    DataType::UInt8 => TsFieldType::Number,
    DataType::UInt16 => TsFieldType::Number,
    DataType::UInt32 => TsFieldType::Number,
    DataType::UInt64 => TsFieldType::Number,
    DataType::UInt128 => TsFieldType::Number,
    DataType::UInt256 => TsFieldType::Number,
    DataType::Signed => TsFieldType::Number,
    DataType::SignedInteger => TsFieldType::Number,
    DataType::Unsigned => TsFieldType::Number,
    DataType::UnsignedInteger => TsFieldType::Number,

    // Boolean types
    DataType::Boolean => TsFieldType::Boolean,
    DataType::Bool => TsFieldType::Boolean,

    // Date/Time types
    DataType::Date => TsFieldType::Date,
    DataType::Date32 => TsFieldType::Date,
    DataType::Time(_, _) => TsFieldType::Date,
    DataType::Datetime(_) => TsFieldType::Date,
    DataType::Datetime64(_, _) => TsFieldType::Date,
    DataType::Timestamp(_, _) => TsFieldType::String,
    DataType::TimestampNtz => TsFieldType::String,
    DataType::Interval { .. } => TsFieldType::Unknown,

    // JSON types
    DataType::JSON => TsFieldType::Object,
    DataType::JSONB => TsFieldType::Object,

    // PostgreSQL specific types
    DataType::Regclass => TsFieldType::String,
    DataType::GeometricType(_) => TsFieldType::Object,
    DataType::TsVector => TsFieldType::String,
    DataType::TsQuery => TsFieldType::String,

    // Complex types
    DataType::Custom(_, _) => TsFieldType::Unknown,
    DataType::Array(array_element_type_def) => match array_element_type_def {
      sqlparser::ast::ArrayElemTypeDef::None => TsFieldType::Array(Box::new(TsFieldType::Unknown)),
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
    DataType::Map(_, _) => TsFieldType::Object,
    DataType::Tuple(_) => TsFieldType::Object,
    DataType::Nested(_) => TsFieldType::Object,
    DataType::Enum(_, _) => TsFieldType::Array(Box::new(TsFieldType::String)),
    DataType::Set(_) => TsFieldType::Array(Box::new(TsFieldType::String)),
    DataType::Struct(_, _) => TsFieldType::Object,
    DataType::Union(_) => TsFieldType::Object,

    // ClickHouse wrapper types
    DataType::Nullable(inner_type) => translate_data_type(inner_type),
    DataType::LowCardinality(inner_type) => translate_data_type(inner_type),

    // Special types
    DataType::Unspecified => TsFieldType::Unknown,
    DataType::Trigger => TsFieldType::Unknown,
    DataType::AnyType => TsFieldType::Any,
  }
}
