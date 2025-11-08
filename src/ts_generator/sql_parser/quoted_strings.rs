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
    // We need to extract the Ident from the first ObjectNamePart
    let first_part = &self.0.0[0];
    if let Some(ident) = first_part.as_ident() {
      let quote_style = &ident.quote_style;
      let name = &ident.value;
      let name = trim_table_name(name, quote_style);
      write!(f, "{name}")
    } else {
      // Fallback: if it's a function-based name, use the default Display implementation
      write!(f, "{}", first_part)
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
