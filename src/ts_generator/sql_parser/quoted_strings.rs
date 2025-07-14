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
    let quote_style = &self.0 .0[0].quote_style;
    let name = &self.0 .0[0].value;
    let name = trim_table_name(name, quote_style);
    write!(f, "{name}")
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
