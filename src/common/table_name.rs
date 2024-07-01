pub trait TrimQuotes {
    fn trim_table_name(&self, quote_style: char) -> String;
}

impl TrimQuotes for String {
    fn trim_table_name(&self, quote_style: char) -> String {
        self.trim_start_matches(quote_style).trim_end_matches(quote_style).to_owned()
    }
}
