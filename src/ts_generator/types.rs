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

impl fmt::Display for TsFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsFieldType::Boolean => write!(f, "{}", "boolean".to_string()),
            TsFieldType::Number => write!(f, "{}", "number".to_string()),
            TsFieldType::String => write!(f, "{}", "string".to_string()),
            TsFieldType::Object => write!(f, "{}", "object".to_string()),
            TsFieldType::Any => write!(f, "{}", "any".to_string()),
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
}

pub struct TsQuery {
    pub name: String,
    pub params: HashMap<String, TsFieldType>,
    pub result: HashMap<String, TsFieldType>,
}

impl TsQuery {
    fn fmt_attributes_map(
        &self,
        f: &mut fmt::Formatter<'_>,
        attrs_map: &HashMap<String, TsFieldType>,
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
