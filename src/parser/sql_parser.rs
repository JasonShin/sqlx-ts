use crate::common::SQL;
use color_eyre::eyre::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lrc,
  SourceMap, DUMMY_SP,
};

pub fn parse_sql_file(path: &PathBuf) -> Result<(HashMap<PathBuf, Vec<SQL>>, Handler)> {
  let contents = fs::read_to_string(path)?;
  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  let mut sqls_map: HashMap<PathBuf, Vec<SQL>> = HashMap::new();
  let sqls = extract_sql_queries_from_file(&contents, path)?;

  if !sqls.is_empty() {
    sqls_map.insert(path.clone(), sqls);
  }

  Ok((sqls_map, handler))
}

/// Extract SQL queries from raw SQL file content
/// Supports multiple queries separated by semicolons and annotations
fn extract_sql_queries_from_file(content: &str, file_path: &Path) -> Result<Vec<SQL>> {
  let mut queries = Vec::new();

  // Split content by semicolons to handle multiple queries
  let query_blocks = split_sql_queries(content);

  for (index, block) in query_blocks.iter().enumerate() {
    let trimmed_block = block.trim();

    // Skip empty blocks or comment-only blocks
    if trimmed_block.is_empty() || is_comment_only(trimmed_block) {
      continue;
    }

    // Extract annotations and clean query
    let (query_name, _db_connection, cleaned_query) = extract_annotations_from_sql(trimmed_block);

    // Skip if no actual SQL content after cleaning
    if cleaned_query.trim().is_empty() {
      continue;
    }

    // Generate default name if not provided via annotation
    let var_decl_name = query_name.or_else(|| generate_default_query_name(file_path, index));

    let sql = SQL {
      query: cleaned_query,
      var_decl_name,
      span: DUMMY_SP.into(),
    };

    queries.push(sql);
  }

  Ok(queries)
}

/// Split SQL content into individual queries
/// Handles semicolons inside strings and comments properly
fn split_sql_queries(content: &str) -> Vec<String> {
  let mut queries = Vec::new();
  let mut current_query = String::new();
  let mut in_string = false;
  let mut in_comment = false;
  let mut string_delimiter = '\0';
  let mut chars = content.chars().peekable();

  while let Some(ch) = chars.next() {
    match ch {
      // Handle string literals
      '\'' | '"' if !in_comment => {
        if !in_string {
          in_string = true;
          string_delimiter = ch;
        } else if ch == string_delimiter {
          // Check for escaped quotes
          if chars.peek() == Some(&ch) {
            current_query.push(ch);
            current_query.push(chars.next().unwrap()); // consume the escaped quote
            continue;
          } else {
            in_string = false;
          }
        }
        current_query.push(ch);
      }

      // Handle single-line comments
      '-' if !in_string && !in_comment => {
        if chars.peek() == Some(&'-') {
          in_comment = true;
          current_query.push(ch);
          current_query.push(chars.next().unwrap()); // consume second dash
        } else {
          current_query.push(ch);
        }
      }

      // Handle multi-line comments
      '/' if !in_string && !in_comment => {
        if chars.peek() == Some(&'*') {
          in_comment = true;
          current_query.push(ch);
          current_query.push(chars.next().unwrap()); // consume asterisk
        } else {
          current_query.push(ch);
        }
      }

      '*' if in_comment && !in_string => {
        current_query.push(ch);
        if chars.peek() == Some(&'/') {
          current_query.push(chars.next().unwrap()); // consume slash
          in_comment = false;
        }
      }

      // Handle newlines (end single-line comments)
      '\n' | '\r' if in_comment => {
        in_comment = false;
        current_query.push(ch);
      }

      // Handle semicolons (query separators)
      ';' if !in_string && !in_comment => {
        current_query.push(ch);
        queries.push(current_query.clone());
        current_query.clear();
      }

      _ => {
        current_query.push(ch);
      }
    }
  }

  // Add the last query if it doesn't end with semicolon
  if !current_query.trim().is_empty() {
    queries.push(current_query);
  }

  queries
}

/// Check if a block contains only comments and whitespace
fn is_comment_only(block: &str) -> bool {
  let lines: Vec<&str> = block.lines().collect();

  for line in lines {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      continue;
    }

    // Check if line starts with comment markers
    if !trimmed.starts_with("--") && !trimmed.starts_with("/*") && !trimmed.starts_with("*") {
      return false;
    }
  }

  true
}

/// Extract annotations from SQL content and return cleaned query
/// Supports @name and @db annotations
fn extract_annotations_from_sql(content: &str) -> (Option<String>, Option<String>, String) {
  let mut query_name = None;
  let mut db_connection = None;
  let mut cleaned_lines = Vec::new();

  // Regex patterns for annotations
  let name_re = Regex::new(r"@name:\s*(.+)").unwrap();
  let db_re = Regex::new(r"@db:\s*(.+)").unwrap();

  for line in content.lines() {
    let trimmed_line = line.trim();

    // Skip empty lines
    if trimmed_line.is_empty() {
      cleaned_lines.push(line);
      continue;
    }

    // Check for name annotation in comments
    if let Some(stripped) = trimmed_line.strip_prefix("--") {
      let comment_content = &stripped.trim();

      if let Some(captures) = name_re.captures(comment_content) {
        query_name = Some(captures.get(1).unwrap().as_str().trim().to_string());
        continue; // Don't include annotation lines in final query
      }

      if let Some(captures) = db_re.captures(comment_content) {
        db_connection = Some(captures.get(1).unwrap().as_str().trim().to_string());
        continue; // Don't include annotation lines in final query
      }
    }

    // Check for annotations in multi-line comments
    if trimmed_line.starts_with("/*") || trimmed_line.contains("@name:") || trimmed_line.contains("@db:") {
      if let Some(captures) = name_re.captures(trimmed_line) {
        query_name = Some(captures.get(1).unwrap().as_str().trim().to_string());
      }

      if let Some(captures) = db_re.captures(trimmed_line) {
        db_connection = Some(captures.get(1).unwrap().as_str().trim().to_string());
      }

      // Only skip pure annotation lines
      if trimmed_line.starts_with("/*")
        && (trimmed_line.contains("@name:") || trimmed_line.contains("@db:"))
        && trimmed_line.ends_with("*/")
      {
        continue;
      }
    }

    cleaned_lines.push(line);
  }

  let cleaned_query = cleaned_lines.join("\n");
  (query_name, db_connection, cleaned_query)
}

/// Generate a default query name from file path and index
fn generate_default_query_name(file_path: &Path, index: usize) -> Option<String> {
  let file_stem = file_path.file_stem()?.to_str()?;

  // Convert to camelCase and add index if multiple queries
  let base_name = file_stem.replace(['-', '_'], " ");
  let base_name = to_camel_case(&base_name);

  if index == 0 {
    Some(base_name)
  } else {
    Some(format!("{}{}", base_name, index + 1))
  }
}

/// Convert string to camelCase
fn to_camel_case(s: &str) -> String {
  let words: Vec<&str> = s.split_whitespace().collect();
  if words.is_empty() {
    return String::new();
  }

  let mut result = words[0].to_lowercase();
  for word in &words[1..] {
    if !word.is_empty() {
      let mut chars = word.chars();
      if let Some(first) = chars.next() {
        result.push(first.to_ascii_uppercase());
        result.push_str(&chars.as_str().to_lowercase());
      }
    }
  }

  result
}
