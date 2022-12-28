use crate::ts_generator::types::TsFieldType;
use regex::Regex;
use std::collections::HashMap;

pub fn extract_result_annotations(query: &String) -> HashMap<String, Vec<TsFieldType>> {
    let re = Regex::new(r"@result (\w+) -> ([^\n]+)").unwrap();
    let captures = re.captures_iter(query.as_str());

    let mut result: HashMap<String, Vec<TsFieldType>> = HashMap::new();
    for capture in captures {
        let name = capture.get(1);
        let types = capture.get(2);
        match (name, types) {
            (Some(name), Some(types)) => {
                let name = name.as_str().to_string();
                let types = types
                    .as_str()
                    .split('|')
                    .map(|t| t.trim())
                    .map(TsFieldType::get_ts_field_from_annotation)
                    .collect::<Vec<TsFieldType>>();

                result.insert(name, types);
            }
            (_, _) => {}
        }
    }

    result
}
