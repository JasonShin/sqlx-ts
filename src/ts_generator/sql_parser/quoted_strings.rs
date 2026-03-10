use sqlparser::ast::{Ident, ObjectName, TableAlias};
use std::fmt;
use std::fmt::Formatter;

fn trim_table_name(val: &String, quote_style: &Option<char>) -> String {
  if quote_style.is_none() {
    return val.to_owned();
  }
  let quote_style = quote_style.unwrap();
  val
    .trim_start_matches(quote_style)
    .trim_end_matches(quote_style)
    .to_owned()
}

pub struct DisplayIndent<'a>(pub &'a Ident);

impl fmt::Display for DisplayIndent<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let quote_style = &self.0.quote_style;
    let name = &self.0.value;
    let name = trim_table_name(name, quote_style);
    write!(f, "{name}")
  }
}

pub struct DisplayObjectName<'a>(pub &'a ObjectName);

impl fmt::Display for DisplayObjectName<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    // In sqlparser 0.59.0, ObjectName contains Vec<ObjectNamePart> instead of Vec<Ident>
    // For qualified table names (e.g., database.schema.table), we want the last identifier (table name)
    let last_part = self.0 .0.last().expect(
      "ObjectName must contain at least one part (sqlparser invariant).\
       If you're seeing this, it's a bug in sqlparser or the SQL parsing logic.",
    );

    if let Some(ident) = last_part.as_ident() {
      let quote_style = &ident.quote_style;
      let name = &ident.value;
      let name = trim_table_name(name, quote_style);
      write!(f, "{name}")
    } else {
      // Fallback: if it's a function-based name, use the default Display implementation
      write!(f, "{}", last_part)
    }
  }
}

pub struct DisplayTableAlias<'a>(pub &'a TableAlias);

impl fmt::Display for DisplayTableAlias<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let quote_style = &self.0.name.quote_style;
    let name = &self.0.name.value;
    let name = trim_table_name(name, quote_style);
    write!(f, "{name}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use sqlparser::ast::{SetExpr, Statement, TableFactor};
  use sqlparser::dialect::PostgreSqlDialect;
  use sqlparser::parser::Parser;

  fn parse_table_name(sql: &str) -> ObjectName {
    let dialect = PostgreSqlDialect {};
    let statements = Parser::parse_sql(&dialect, sql).expect("Failed to parse SQL");
    let statement = &statements[0];

    if let Statement::Query(query) = statement {
      if let SetExpr::Select(select) = query.body.as_ref() {
        if let Some(table_with_joins) = select.from.first() {
          if let TableFactor::Table { name, .. } = &table_with_joins.relation {
            return name.clone();
          }
        }
      }
    }
    panic!("Could not extract table name from SQL");
  }

  #[test]
  fn test_display_object_name_single_identifier() {
    // Test simple table name: "users"
    let object_name = parse_table_name("SELECT * FROM users");
    let display = DisplayObjectName(&object_name);
    assert_eq!(display.to_string(), "users");
  }

  #[test]
  fn test_display_object_name_qualified_schema_table() {
    // Test qualified name: "public.users" -> should return "users"
    let object_name = parse_table_name("SELECT * FROM public.users");
    let display = DisplayObjectName(&object_name);
    assert_eq!(display.to_string(), "users");
  }

  #[test]
  fn test_display_object_name_qualified_database_schema_table() {
    // Test fully qualified name: "database.schema.table" -> should return "table"
    let object_name = parse_table_name("SELECT * FROM mydb.public.users");
    let display = DisplayObjectName(&object_name);
    assert_eq!(display.to_string(), "users");
  }

  #[test]
  fn test_display_object_name_with_quotes() {
    // Test quoted identifier: `"my_table"` -> should return "my_table" without quotes
    let object_name = parse_table_name("SELECT * FROM \"my_table\"");
    let display = DisplayObjectName(&object_name);
    assert_eq!(display.to_string(), "my_table");
  }

  #[test]
  fn test_display_object_name_qualified_with_quotes() {
    // Test qualified with quotes: `"public"."my_table"` -> should return "my_table"
    let object_name = parse_table_name("SELECT * FROM \"public\".\"my_table\"");
    let display = DisplayObjectName(&object_name);
    assert_eq!(display.to_string(), "my_table");
  }
}
