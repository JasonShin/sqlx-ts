// LIST OF FUNCTIONS FOUND https://www.w3schools.com/mysql/mysql_ref_functions.asp
pub static NUMERIC_FUNCTIONS: &[&str] = &[
  "ABS", "ACOS", "ASIN", "ATAN", "ATAN2", "AVG", "CEIL", "CEILING", "COS", "COT", "COUNT", "DEGREES", "DIV", "EXP",
  "FLOOR", "GREATEST", "LEAST", "LN", "LOG", "LOG10", "LOG2", "MAX", "MIN", "MOD", "PI", "POW", "POWER", "RADIANS",
  "RAND", "ROUND", "SIGN", "SIN", "SQRT", "SUM", "TAN", "TRUNCATE", "TRUNC",
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
