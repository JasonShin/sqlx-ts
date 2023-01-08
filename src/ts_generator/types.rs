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
pub enum ArrayItem {
    String,
    Number,
    Boolean,
    Object,
    Null,
    Any,
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
    Array(ArrayItem),
}

impl fmt::Display for TsFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsFieldType::Boolean => write!(f, "boolean"),
            TsFieldType::Number => write!(f, "number"),
            TsFieldType::String => write!(f, "string"),
            TsFieldType::Object => write!(f, "object"),
            TsFieldType::Any => write!(f, "any"),
            TsFieldType::Null => write!(f, "null"),
            TsFieldType::Never => write!(f, "never"),
            TsFieldType::Array(ts_field_type) => match ts_field_type {
                ArrayItem::String => write!(f, "Array<string>"),
                ArrayItem::Number => write!(f, "Array<number>"),
                ArrayItem::Boolean => write!(f, "Array<boolean>"),
                ArrayItem::Object => write!(f, "Array<object>"),
                ArrayItem::Null => write!(f, "Array<null>"),
                ArrayItem::Any => write!(f, "Array<any>"),
            },
        }
    }
}

impl TsFieldType {
    pub fn to_array_item(self) -> Self {
        match self {
            TsFieldType::String => TsFieldType::Array(ArrayItem::String),
            TsFieldType::Number => TsFieldType::Array(ArrayItem::Number),
            TsFieldType::Boolean => TsFieldType::Array(ArrayItem::Boolean),
            TsFieldType::Object => TsFieldType::Array(ArrayItem::Object),
            TsFieldType::Null => TsFieldType::Array(ArrayItem::Null),
            TsFieldType::Any => TsFieldType::Array(ArrayItem::Any),
            TsFieldType::Never => panic!("Cannot convert never to an array of never"),
            TsFieldType::Array(arr) => TsFieldType::Array(arr),
        }
    }

    pub fn get_ts_field_type_from_mysql_field_type(mysql_field_type: String) -> Self {
        // TODO: Cover all mysql_field_types
        if mysql_field_type == "varchar" {
            return Self::String;
        }
        if mysql_field_type == "int" {
            return Self::Number;
        }
        if mysql_field_type == "smallint" {
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
        Self::Any
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
    /// inserts a value into the result hashmap
    /// it should only insert a value if you are working with a non-subquery queries
    pub fn insert_result(&mut self, key: String, value: &Vec<TsFieldType>, is_subquery: bool) {
        if !is_subquery {
            let _ = self.result.insert(key, value.clone());
        }
    }

    pub fn insert_param(&mut self, value: &TsFieldType) {
        self.params.push(*value)
    }

    fn fmt_params(&self, _: &mut fmt::Formatter<'_>, params: &Vec<TsFieldType>) -> String {
        let result = params
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        result
    }

    fn fmt_result(&self, _f: &mut fmt::Formatter<'_>, attrs_map: &HashMap<String, Vec<TsFieldType>>) -> String {
        let mut keys = Vec::from_iter(attrs_map.keys());
        keys.sort();

        let result: Vec<String> = keys
            .iter()
            .map(|key| {
                let data_type = attrs_map.get(key.to_owned()).unwrap();
                let data_types = data_type
                    .iter()
                    .map(|ts_field_type| ts_field_type.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ");
                format!("{key}: {data_types};")
            })
            .collect();

        result.join("\n\t")
    }
}

impl fmt::Display for TsQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let params_str = self.fmt_params(f, &self.params);
        let result_str = self.fmt_result(f, &self.result);

        let params = format!(
            r"
export type {name}Params = [{params_str}];
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
    params: {name}Params;
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
