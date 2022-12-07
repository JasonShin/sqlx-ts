use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self};

use mysql::Conn as MySQLConn;
use postgres::Client as PostgresConn;

pub enum DBConn<'a> {
    // TODO: Maybe we can also pass down db_name through DBConn
    MySQLPooledConn(&'a mut RefCell<&'a mut MySQLConn>),
    PostgresConn(&'a mut RefCell<&'a mut PostgresConn>),
}

#[derive(Debug, Clone, Copy)]
pub enum TsFieldType {
    String,
    Number,
    Boolean,
    Object,
    Null,
    Any,
    Never,
}

impl fmt::Display for TsFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsFieldType::Boolean => write!(f, "{}", "boolean".to_string()),
            TsFieldType::Number => write!(f, "{}", "number".to_string()),
            TsFieldType::String => write!(f, "{}", "string".to_string()),
            TsFieldType::Object => write!(f, "{}", "object".to_string()),
            TsFieldType::Any => write!(f, "{}", "any".to_string()),
            TsFieldType::Null => write!(f, "{}", "null".to_string()),
            TsFieldType::Never => write!(f, "{}", "never".to_string()),
        }
    }
}

impl TsFieldType {
    pub fn get_ts_field_type_from_mysql_field_type(mysql_field_type: String) -> Self {
        // TODO: Cover all mysql_field_types
        if mysql_field_type == "varchar" {
            return Self::String;
        } else if mysql_field_type == "int" {
            return Self::Number;
        } else if mysql_field_type == "smallint" {
            return Self::Number;
        }

        Self::Any
    }

    pub fn get_ts_field_from_annotation(annotated_type: &str) -> Self {
        if annotated_type == "string" {
            return Self::String;
        } else if annotated_type == "number" {
            return Self::Number;
        } else if annotated_type == "boolean" {
            return Self::Boolean;
        } else if annotated_type == "object" {
            return Self::Object;
        } else if annotated_type == "null" {
            return Self::Null;
        }
        return Self::Any;
    }
}

#[derive(Debug)]
pub struct TsQuery {
    pub name: String,
    pub params: Vec<TsFieldType>,
    // TODO: Need a type for List of Params strongly typed in order
    pub result: HashMap<String, Vec<TsFieldType>>,
}

impl TsQuery {
    fn fmt_params(&self, f: &mut fmt::Formatter<'_>, params: &Vec<TsFieldType>) -> String {
        let result = params.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{}", result)
    }

    fn fmt_result(&self, f: &mut fmt::Formatter<'_>, attrs_map: &HashMap<String, Vec<TsFieldType>>) -> String {
        let result: Vec<String> = attrs_map
            .into_iter()
            .map(|(name, data_type)| {
                let data_types = data_type
                    .into_iter()
                    .map(|ts_field_type| ts_field_type.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ");
                format!("{name}: {data_types};")
            })
            .collect();

        format!("{}", result.join("\n\t").to_string())
    }
}

impl fmt::Display for TsQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let params_str = self.fmt_params(f, &self.params);
        let result_str = self.fmt_result(f, &self.result);

        let params = format!(
            r"
export interface I{name}Params {{
    {params_str}
}};
"
        );

        let result = format!(
            r"
export interface I{name}Result {{
    {result_str}
}};
"
        );

        let query = format!(
            r"
export interface I{name}Query {{
    params: I{name}Params;
    result: I{name}Result;
}};
"
        );

        let final_code = format!(
            r"
{params}
{result}
{query}"
        );

        writeln!(f, "{}", final_code)
    }
}
