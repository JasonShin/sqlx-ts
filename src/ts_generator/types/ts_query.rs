use crate::common::logger::*;
use color_eyre::eyre::Result;
use convert_case::{Case, Casing};
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{self};

use crate::common::lazy::CONFIG;
use crate::ts_generator::errors::TsGeneratorError;

type Array2DContent = Vec<Vec<TsFieldType>>;

#[derive(Debug, Clone, PartialEq)]
pub enum TsFieldType {
  String,
  Number,
  Boolean,
  Object,
  Date,
  Null,
  Enum(Vec<String>),
  Any,
  Array2D(Array2DContent),
  Array(Box<TsFieldType>),
  Never,
}

impl fmt::Display for TsFieldType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TsFieldType::Boolean => write!(f, "boolean"),
      TsFieldType::Number => write!(f, "number"),
      TsFieldType::String => write!(f, "string"),
      TsFieldType::Object => write!(f, "object"),
      TsFieldType::Date => write!(f, "Date"),
      TsFieldType::Any => write!(f, "any"),
      TsFieldType::Null => write!(f, "null"),
      TsFieldType::Never => write!(f, "never"),
      TsFieldType::Array(ts_field_type) => {
        let ts_field_type = ts_field_type.clone();
        let ts_field_type = *ts_field_type;
        write!(f, "Array<{ts_field_type}>")
      }
      TsFieldType::Array2D(nested_array) => {
        let result = nested_array
          .iter()
          .map(|items| {
            let items = items
              .iter()
              .map(|x| x.to_string())
              .collect::<Vec<String>>()
              .join(", ");

            format!("[{items}]")
          })
          .collect::<Vec<String>>()
          .join(", ");

        write!(f, "{result}")
      }
      TsFieldType::Enum(values) => {
        let enums: Vec<String> = values.iter().map(|x| format!("'{x}'")).collect();
        let joined_enums = enums.join(" | ");
        write!(f, "{joined_enums}")
      }
    }
  }
}

impl TsFieldType {
  /// The method is to convert the data_type field that you get from PostgreSQL as strings into TsFieldType
  /// so when we stringify TsFieldType, we can correctly translate the data_type into the corresponding TypeScript
  /// data type
  ///
  /// @examples
  /// get_ts_field_type_from_postgres_field_type("integer") -> TsFieldType::Number
  /// get_ts_field_type_from_postgres_field_type("smallint") -> TsFieldType::Number
  /// get_ts_field_type_from_postgres_field_type("character varying",  ",,")
  ///
  pub fn get_ts_field_type_from_postgres_field_type(
    field_type: String,
    field_name: String,
    table_name: String,
    enum_values: Option<Vec<String>>,
  ) -> Self {
    match field_type.as_str() {
      "smallint" | "integer" | "real" | "double precision" | "numeric" => Self::Number,
      "character" | "character varying" | "bytea" | "uuid" | "text" => Self::String,
      "boolean" => Self::Boolean,
      "json" | "jsonb" => Self::Object,
      "ARRAY" | "array" => Self::Any,
      "date" => Self::Date,
      "USER-DEFINED" => {
        if let Some(enum_values) = enum_values {
          return Self::Enum(enum_values);
        }
        let warning_message = format!("Failed to find enum values for field {field_name} of table {table_name}");
        warning!(warning_message);
        Self::Any
      }
      _ => Self::Any,
    }
  }

  pub fn get_ts_field_type_from_mysql_field_type(
    mysql_field_type: String,
    table_name: String,
    field_name: String,
    enum_values: Option<Vec<String>>,
  ) -> Self {
    match mysql_field_type.as_str() {
      "bigint" | "decimal" | "double" | "float" | "int" | "mediumint" | "smallint" | "year" => Self::Number,
      "binary" | "bit" | "blob" | "char" | "text" | "varbinary" | "varchar" => Self::String,
      "tinyint" => Self::Boolean,
      "date" | "datetime" | "timestamp" => Self::Date,
      "enum" => {
        if let Some(enum_values) = enum_values {
          return Self::Enum(enum_values);
        }

        let warning_message = format!("Failed to find enum values for field {field_name} of table {table_name}");
        warning!(warning_message);
        Self::Any
      }
      _ => Self::Any,
    }
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

/// TsQuery holds information required to generate typescript type definition
/// of the target SQL query
///
/// There are tests under `tests` folder that checks TsQuery generates the
/// correct type definitions
#[derive(Debug, Clone)]
pub struct TsQuery {
  pub name: String,
  param_order: i32,
  // We use BTreeMap here as it's a collection that's already sorted
  // TODO: use usize instead
  pub params: BTreeMap<usize, Vec<TsFieldType>>,
  pub annotated_params: BTreeMap<usize, TsFieldType>,

  // We use BTreeMap here as it's a collection that's already sorted
  pub insert_params: BTreeMap<usize, BTreeMap<usize, TsFieldType>>,
  // Holds any annotated @param and perform replacement when generated TS types
  pub annotated_insert_params: BTreeMap<usize, BTreeMap<usize, TsFieldType>>,

  pub result: HashMap<String, Vec<TsFieldType>>,
  // Holds any annotated @result and perform replacement when generating TS types
  pub annotated_results: HashMap<String, Vec<TsFieldType>>,
}

impl TsQuery {
  pub fn new(name: String) -> TsQuery {
    TsQuery {
      name,
      param_order: 0,
      params: BTreeMap::new(),
      annotated_params: BTreeMap::new(),
      result: HashMap::new(),
      insert_params: BTreeMap::new(),
      annotated_results: HashMap::new(),
      annotated_insert_params: BTreeMap::new(),
    }
  }

  /// set annotatd results to ts query so when generating ts types, it can use annotated results wherever possible
  pub fn set_annotated_results(&mut self, annotated_results: HashMap<String, Vec<TsFieldType>>) {
    self.annotated_results = annotated_results;
  }

  pub fn set_annotated_params(&mut self, annotated_params: BTreeMap<usize, TsFieldType>) {
    self.annotated_params = annotated_params;
  }

  pub fn format_column_name(&self, column_name: &str) -> String {
    let convert_to_camel_case_column_name = &CONFIG
      .generate_types_config
      .to_owned()
      .map(|x| x.convert_to_camel_case_column_name);

    let column_naming_convention = &CONFIG
      .generate_types_config
      .to_owned()
      .and_then(|x| x.column_naming_convention);

    if column_naming_convention.is_some() {
      let column_name_convention = &column_naming_convention.clone().unwrap();
      column_name_convention.convert(column_name)
    } else {
      match convert_to_camel_case_column_name {
        Some(true) => column_name.to_case(Case::Camel),
        Some(false) | None => column_name.to_string(),
      }
    }
  }

  /// inserts a value into the result hashmap
  /// it should only insert a value if you are working with a non-subquery queries
  pub fn insert_result(
    &mut self,
    alias: Option<&str>,
    value: &[TsFieldType],
    is_selection: bool,
    is_nullable: bool,
    expr_for_logging: &str,
  ) -> Result<(), TsGeneratorError> {
    if is_selection {
      if let Some(alias) = alias {
        let temp_alias = alias;
        let alias = &self.format_column_name(alias);
        let value = &mut self
          .annotated_results
          .get(temp_alias)
          .cloned()
          .unwrap_or_else(|| value.to_vec());

        if is_nullable {
          value.push(TsFieldType::Null);
        }

        let _ = &self.result.insert(alias.to_owned(), value.to_owned());
      } else {
        return Err(TsGeneratorError::MissingAliasForFunctions(expr_for_logging.to_string()));
      }
    }
    Ok(())
  }

  /// This is used to insert value params required for INSERT statements
  /// For example if you are given
  ///
  /// e.g.
  /// INSERT INTO table (id, name, address) VALUES (?, 'TEST', ?)
  ///
  /// If you process above MySQL query, it should generate
  ///
  /// e.g.
  /// [ [number, string] ]
  ///
  /// If you are given a query with multiple input values
  ///
  /// e.g.
  /// INSERT INTO table (id, name, address) VALUES (?, 'test', ?), (?, ?, 'address')
  ///
  /// e.g.
  /// [ [number, string], [number, string] ]
  pub fn insert_value_params(&mut self, value: &TsFieldType, point: &(usize, usize), _placeholder: &Option<String>) {
    let (row, column) = point;
    let annotated_insert_param = self.annotated_insert_params.get(row);

    if let Some(annotated_insert_param) = annotated_insert_param {
      let _ = self.insert_params.insert(*row, annotated_insert_param.clone());
    } else {
      let mut row_params = self.insert_params.get_mut(row);

      // If the row of the insert params is not found, create a new BTreeMap and insert it
      if row_params.is_none() {
        let _ = &self.insert_params.insert(*row, BTreeMap::new());
        row_params = self.insert_params.get_mut(row);
      }

      row_params.unwrap().insert(*column, value.to_owned());
    }
  }

  /// Inserts a parameter into TsQuery for type definition generation
  /// If you pass in the order argument, it will use the manually passed in order
  /// It's important to make sure that you are not mixing up the usage
  /// You can only sequentially use `insert_param` with manual order or automatic order parameter
  ///
  /// This method was specifically designed with an assumption that 1 TsQuery is connected to 1 type of DB
  pub fn insert_param(
    &mut self,
    value: &TsFieldType,
    is_nullable: &bool,
    placeholder: &Option<String>,
  ) -> Result<(), TsGeneratorError> {
    if let Some(placeholder) = placeholder {
      let pg_placeholder_pattern = Regex::new(r"\$(\d+)").unwrap();
      let mut values = vec![];

      let order = if placeholder == "?" {
        self.param_order += 1;
        self.param_order
      } else if let Some(caps) = pg_placeholder_pattern.captures(placeholder) {
        caps.get(1)
          .and_then(|m| m.as_str().parse::<i32>().ok())
          .ok_or(TsGeneratorError::UnknownPlaceholder(format!(
            "{placeholder} is not a valid placeholder parameter in PostgreSQL"
          )))? as i32
      } else {
        // No pattern matches the provided placeholder, simply exit out of the function
        return Ok(())
      } as usize;

      if let Some(annotated_param) = self.annotated_params.get(&order) {
        values.push(annotated_param.clone());
      } else {
        values.push(value.clone());
      }

      // Add nullability if required
      if *is_nullable {
        values.push(TsFieldType::Null);
      }

      // Insert values into the parameter map
      self.params.insert(order, values);
    }
    Ok(())
  }

  /// The method is to format SQL params extracted via translate methods
  /// It can work for SELECT, INSERT, DELETE and UPDATE queries
  fn fmt_params(&self, _: &mut fmt::Formatter<'_>) -> String {
    if !self.insert_params.is_empty() {
      return self
        .insert_params
        .values()
        .map(|row| {
          // Process each row and produce Number, String, Boolean
          row
            .iter()
            .map(|(_j, col)| col.to_string())
            .collect::<Vec<String>>()
            .join(", ")
        })
        // Wrap the result of row .to_string in `[]`
        .map(|row| format!("[{}]", row))
        .collect::<Vec<String>>()
        .join(", ");
    }

    // Otherwise we should be processing non-insert query params
    self
      .params
      .values()
      .map(|x| x.iter().map(ToString::to_string).collect::<Vec<String>>().join(" | "))
      .collect::<Vec<String>>()
      .join(", ")
  }

  fn fmt_result(&self, _f: &mut fmt::Formatter<'_>) -> String {
    let mut keys = Vec::from_iter(self.result.keys());
    keys.sort();

    let result: Vec<String> = keys
      .iter()
      .map(|key| {
        let data_type = self.result.get(key.to_owned()).unwrap();
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
    let params_str = self.fmt_params(f);
    let result_str = self.fmt_result(f);

    let params = format!(
      "export type {name}Params = [{params_str}];"
    );

    let result = format!(
      "export interface I{name}Result {{\n\t{result_str}\n}};"
    );

    let query = format!(
      "export interface I{name}Query {{\n\tparams: {name}Params;\n\tresult: I{name}Result;\n}};"
    );

    let final_code = format!(
      "{params}\n\n{result}\n\n{query}"
    );

    writeln!(f, "{}", final_code)
  }
}
