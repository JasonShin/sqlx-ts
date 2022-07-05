use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self};
use std::rc::Rc;

use mysql::Conn;

pub enum DBConn<'a> {
    MySQLPooledConn(&'a mut RefCell<&'a mut Conn>),
}

#[derive(Debug, Clone, Copy)]
pub enum TsFieldType {
    String,
    Number,
    Boolean,
    Object,
    Any,
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
}

#[derive(Debug)]
pub enum TsDataType {
    Boolean,
    Number,
    String,
}

impl fmt::Display for TsDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsDataType::Boolean => write!(f, "{}", "boolean".to_string()),
            TsDataType::Number => write!(f, "{}", "number".to_string()),
            TsDataType::String => write!(f, "{}", "string".to_string()),
        }
    }
}

pub struct TsQuery {
    pub name: String,
    pub params: HashMap<String, TsDataType>,
    pub result: HashMap<String, TsDataType>,
}

impl TsQuery {
    fn fmt_attributes_map(
        &self,
        f: &mut fmt::Formatter<'_>,
        attrs_map: &HashMap<String, TsDataType>,
    ) -> String {
        let result: Vec<String> = attrs_map
            .into_iter()
            .map(|(name, data_type)| format!("{name}: {data_type};"))
            .collect();

        format!("{}", result.join("\n").to_string())
    }
}

impl fmt::Display for TsQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let params_str = self.fmt_attributes_map(f, &self.params);
        let result_str = self.fmt_attributes_map(f, &self.result);

        let params = format!(
            r"
            export interface I{name}Params {{
                {params_str}
            }}
        "
        );

        let result = format!(
            r"
            export interface I{name}Result {{
                {result_str}
            }}
        "
        );

        let query = format!(
            r"
            export interface I{name}Query {{
                params: I{name}Params;
                result: I{name}Result;
            }}
        "
        );

        let final_code = format!(
            r"
            {params};
            {result};
            {query};"
        );

        writeln!(f, "{}", final_code)
    }
}
