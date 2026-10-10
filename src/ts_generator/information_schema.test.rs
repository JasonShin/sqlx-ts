#[cfg(test)]
mod tests {
  use crate::ts_generator::information_schema::{collect_table_schemas, AmbiguousTable, TableSchemas};
  use sqlparser::{dialect::PostgreSqlDialect, parser::Parser};
  use std::collections::HashMap;

  fn collect(sql: &str) -> TableSchemas {
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, sql).unwrap();
    collect_table_schemas(&statements)
  }

  fn schemas(entries: &[(&str, &str)]) -> HashMap<String, String> {
    entries
      .iter()
      .map(|(table, schema)| (table.to_string(), schema.to_string()))
      .collect()
  }

  #[test]
  fn should_ignore_unqualified_tables() {
    assert_eq!(collect("SELECT id FROM items"), TableSchemas::default());
  }

  #[test]
  fn should_collect_schemas_of_qualified_tables() {
    let table_schemas = collect(
      "SELECT a.message FROM staff.announcements a JOIN items i ON i.id = a.id WHERE a.id IN (SELECT id FROM audit.logs)",
    );
    assert_eq!(
      table_schemas.schemas,
      schemas(&[("announcements", "staff"), ("logs", "audit")])
    );
    assert!(table_schemas.ambiguous_tables.is_empty());
  }

  #[test]
  fn should_use_schema_of_database_schema_table_names() {
    let table_schemas = collect("SELECT message FROM postgres.staff.announcements");
    assert_eq!(table_schemas.schemas, schemas(&[("announcements", "staff")]));
  }

  #[test]
  fn should_strip_quotes_from_qualified_names() {
    let table_schemas = collect(r#"SELECT message FROM "staff"."announcements""#);
    assert_eq!(table_schemas.schemas, schemas(&[("announcements", "staff")]));
  }

  #[test]
  fn should_collect_schemas_of_insert_update_and_delete() {
    for sql in [
      "INSERT INTO staff.announcements (message) VALUES ($1)",
      "UPDATE staff.announcements SET message = $1",
      "DELETE FROM staff.announcements WHERE message = $1",
    ] {
      assert_eq!(collect(sql).schemas, schemas(&[("announcements", "staff")]), "{sql}");
    }
  }

  #[test]
  fn should_not_flag_the_same_qualified_table_referenced_twice() {
    let table_schemas = collect("SELECT a.id FROM staff.announcements a JOIN staff.announcements b ON a.id = b.id");
    assert_eq!(table_schemas.schemas, schemas(&[("announcements", "staff")]));
    assert!(table_schemas.ambiguous_tables.is_empty());
  }

  #[test]
  fn should_flag_qualified_and_unqualified_table_with_the_same_name() {
    let table_schemas = collect("SELECT s.message FROM staff.announcements s JOIN announcements p ON p.id = s.id");
    assert_eq!(table_schemas.schemas, schemas(&[("announcements", "staff")]));
    assert_eq!(
      table_schemas.ambiguous_tables,
      vec![AmbiguousTable {
        table_name: "announcements".to_string(),
        references: vec!["staff.announcements".to_string(), "announcements".to_string()],
        resolved_schema: "staff".to_string(),
      }]
    );
  }

  #[test]
  fn should_flag_tables_with_the_same_name_in_different_schemas() {
    let table_schemas = collect("SELECT p.title FROM public.announcements p JOIN staff.announcements s ON p.id = s.id");
    assert_eq!(table_schemas.schemas, schemas(&[("announcements", "public")]));
    assert_eq!(
      table_schemas.ambiguous_tables,
      vec![AmbiguousTable {
        table_name: "announcements".to_string(),
        references: vec!["public.announcements".to_string(), "staff.announcements".to_string()],
        resolved_schema: "public".to_string(),
      }]
    );
  }
}
