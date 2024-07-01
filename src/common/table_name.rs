pub trait TrimQuotes {
    fn trim_table_name(&self, quote_style: Option<char>) -> String;
}

impl TrimQuotes for String {
    fn trim_table_name(&self, quote_style: Option<char>) -> String {
        if quote_style.is_none() {
            return self.to_owned();
        }
        let quote_style = quote_style.unwrap();
        self.trim_start_matches(quote_style)
            .trim_end_matches(quote_style)
            .to_owned()
    }
}
