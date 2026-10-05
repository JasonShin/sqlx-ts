#[cfg(test)]
mod tests {
  use crate::common::config::{CustomTypeMapping, DbConnectionConfig};

  fn connection_with_mapping(type_mapping: &str) -> DbConnectionConfig {
    serde_json::from_str(&format!(r#"{{ "DB_TYPE": "mysql", "type_mapping": {type_mapping} }}"#)).unwrap()
  }

  fn mapped_type(conn: &DbConnectionConfig, db_types: &[&str]) -> Option<String> {
    conn.find_type_mapping(db_types).map(|mapping| match mapping {
      CustomTypeMapping::Simple(type_name) | CustomTypeMapping::WithImport { type_name, .. } => type_name.to_owned(),
    })
  }

  #[test]
  fn should_match_case_insensitively() {
    let conn = connection_with_mapping(r#"{ "BigInt": "string" }"#);
    assert_eq!(mapped_type(&conn, &["BIGINT"]), Some("string".to_string()));
  }

  #[test]
  fn should_ignore_type_modifiers() {
    let conn = connection_with_mapping(r#"{ "varchar": "Name", "bigint unsigned": "string" }"#);
    assert_eq!(mapped_type(&conn, &["VARCHAR(255)"]), Some("Name".to_string()));
    assert_eq!(mapped_type(&conn, &["bigint(20) unsigned"]), Some("string".to_string()));
    assert_eq!(mapped_type(&conn, &["bigint  unsigned"]), Some("string".to_string()));
  }

  #[test]
  fn should_prefer_exact_match_and_earlier_candidates() {
    let conn = connection_with_mapping(r#"{ "tinyint(1)": "boolean", "tinyint": "number", "bigint": "bigint" }"#);
    assert_eq!(
      mapped_type(&conn, &["tinyint(1)", "tinyint"]),
      Some("boolean".to_string())
    );
    assert_eq!(
      mapped_type(&conn, &["tinyint(4)", "tinyint"]),
      Some("number".to_string())
    );
    assert_eq!(
      mapped_type(&conn, &["bigint(20) unsigned", "bigint"]),
      Some("bigint".to_string())
    );
  }

  #[test]
  fn should_return_none_without_mapping() {
    let conn = connection_with_mapping("null");
    assert_eq!(mapped_type(&conn, &["bigint"]), None);
    let conn = connection_with_mapping(r#"{ "bigint": "string" }"#);
    assert_eq!(mapped_type(&conn, &["integer"]), None);
  }

  #[test]
  fn should_parse_object_mapping_with_optional_import() {
    let conn = connection_with_mapping(
      r#"{ "a": { "type": "A" }, "b": { "type": "B", "import": "" }, "c": { "type": "C", "import": "import type { C } from 'c'" } }"#,
    );
    let mapping = conn.type_mapping.unwrap();
    assert!(matches!(&mapping["a"], CustomTypeMapping::Simple(t) if t == "A"));
    assert!(matches!(&mapping["b"], CustomTypeMapping::Simple(t) if t == "B"));
    assert!(matches!(&mapping["c"], CustomTypeMapping::WithImport { type_name, .. } if type_name == "C"));
  }

  #[test]
  fn should_reject_invalid_mappings() {
    for type_mapping in [
      r#"{ "bigint": "" }"#,
      r#"{ "bigint": { "type": " " } }"#,
      r#"{ "bigint": { "import": "import x from 'x'" } }"#,
      r#"{ "bigint": { "type": "X", "import": 1 } }"#,
      r#"{ "bigint": 1 }"#,
    ] {
      let result: Result<DbConnectionConfig, _> =
        serde_json::from_str(&format!(r#"{{ "DB_TYPE": "mysql", "type_mapping": {type_mapping} }}"#));
      assert!(result.is_err(), "Expected {type_mapping} to be rejected");
    }
  }
}
