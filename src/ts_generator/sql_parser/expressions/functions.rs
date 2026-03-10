// LIST OF FUNCTIONS FOUND https://www.w3schools.com/mysql/mysql_ref_functions.asp
pub static NUMERIC_FUNCTIONS: &[&str] = &[
  "ABS",
  "ACOS",
  "ASIN",
  "ATAN",
  "ATAN2",
  "AVG",
  "CEIL",
  "CEILING",
  "COS",
  "COT",
  "COUNT",
  "DEGREES",
  "DIV",
  "EXP",
  "FLOOR",
  "GREATEST",
  "LEAST",
  "LN",
  "LOG",
  "LOG10",
  "LOG2",
  "MAX",
  "MIN",
  "MOD",
  "PI",
  "POW",
  "POWER",
  "RADIANS",
  "RAND",
  "ROUND",
  "SIGN",
  "SIN",
  "SQRT",
  "SUM",
  "TAN",
  "TRUNCATE",
  "TRUNC",
  // Window / ranking functions that always return a numeric value
  "RANK",
  "DENSE_RANK",
  "ROW_NUMBER",
  "NTILE",
  "PERCENT_RANK",
  "CUME_DIST",
];

pub static STRING_FUNCTIONS: &[&str] = &[
  // TODO: These methods expect string as input and number as output
  "ASCII",
  "CHAR_LENGTH",
  "CHARACTER_LENGTH",
  // String to string
  "CONCAT",
  "CONCAT_WS",
  "FIELD",
  "FIND_IN_SET",
  "FORMAT",
  "INSERT",
  "INSTR",
  "LCASE",
  "LEFT",
  "LENGTH",
  "LOCATE",
  "LOWER",
  "LPAD",
  "LTRIM",
  "MID",
  "POSITION",
  "REPEAT",
  "REPLACE",
  "REVERSE",
  "RIGHT",
  "RPAD",
  "RTRIM",
  "SPACE",
  "STRCMP",
  "SUBSTR",
  "SUBSTRING",
  "SUBSTRING_INDEX",
  "TRIM",
  "UCASE",
  "UPPER",
  "ENCODE",
  "TO_CHAR",
  "TO_DATE",
  "TO_TIMESTAMP",
  "DATE_PART",
  "AGE",
  "DATE_TRUNC",
];

pub static DATE_FUNCTIONS: &[&str] = &[
  "ADDDATE",
  "ADDTIME",
  "CURDATE",
  "CURRENT_DATE",
  "CURRENT_TIME",
  "CURRENT_TIMESTAMP",
  "CURTIME",
  "DATE",
  "DATE_ADD",
  "DATE_FORMAT",
  "DATE_SUB",
  "DAY",
  "DAYNAME",
  "DAYOFMONTH",
  "DAYOFWEEK",
  "DAYOFYEAR",
  "EXTRACT",
  "FROM_DAYS",
  "HOUR",
  "LAST_DAY",
  "LOCALTIME",
  "LOCALTIMESTAMP",
  "MAKEDATE",
  "MAKETIME",
  "MICROSECOND",
  "MINUTE",
  "MONTH",
  "MONTHNAME",
  "NOW",
  "QUARTER",
  "SECOND",
  "SEC_TO_TIME",
  "STR_TO_DATE",
  "SUBDATE",
  "SUBTIME",
  "SYSDATE",
  "TIME",
  "TIME_FORMAT",
  "TIME_TO_SEC",
  "TIMEDIFF",
  "TIMESTAMP",
  "TO_DAYS",
  "WEEK",
  "WEEKDAY",
  "WEEKOFYEAR",
  "YEAR",
  "YEARWEEK",
];

pub fn is_numeric_function(func_name: &str) -> bool {
  NUMERIC_FUNCTIONS.contains(&func_name.to_uppercase().as_str())
}

pub fn is_string_function(func_name: &str) -> bool {
  STRING_FUNCTIONS.contains(&func_name.to_uppercase().as_str())
}

pub fn is_date_function(func_name: &str) -> bool {
  DATE_FUNCTIONS.contains(&func_name.to_uppercase().as_str())
}

// Type-polymorphic functions that return the type of their first argument
pub static TYPE_POLYMORPHIC_FUNCTIONS: &[&str] = &[
  "IFNULL",
  "COALESCE",
  "NULLIF",
  "NVL",
  // Window value functions — return the same type as their first argument
  "LAG",
  "LEAD",
  "FIRST_VALUE",
  "LAST_VALUE",
  "NTH_VALUE",
];

pub fn is_type_polymorphic_function(func_name: &str) -> bool {
  TYPE_POLYMORPHIC_FUNCTIONS.contains(&func_name.to_uppercase().as_str())
}

// JSON/JSONB functions that build objects/arrays
pub static JSON_BUILD_FUNCTIONS: &[&str] = &[
  "JSONB_BUILD_OBJECT",
  "JSON_BUILD_OBJECT",
  "JSONB_BUILD_ARRAY",
  "JSON_BUILD_ARRAY",
  "JSON_OBJECT", // MySQL JSON_OBJECT function
];

// JSON/JSONB aggregation functions
pub static JSON_AGG_FUNCTIONS: &[&str] = &[
  "JSONB_AGG",
  "JSON_AGG",
  "JSON_OBJECT_AGG",
  "JSONB_OBJECT_AGG",
  "JSON_ARRAYAGG", // MySQL JSON_ARRAYAGG function
];

pub fn is_json_build_function(func_name: &str) -> bool {
  JSON_BUILD_FUNCTIONS.contains(&func_name.to_uppercase().as_str())
}

pub fn is_json_agg_function(func_name: &str) -> bool {
  JSON_AGG_FUNCTIONS.contains(&func_name.to_uppercase().as_str())
}
