use crate::ts_generator::types::ts_query::TsFieldType;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};

pub fn extract_result_annotations(query: &str) -> HashMap<String, Vec<TsFieldType>> {
    let re = Regex::new(r"@result (\w+): ([^\n]+)").unwrap();
    let captures = re.captures_iter(query);

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
            // We should skip the annotation if it does not match the format sqlx-ts is expecting
            (_, _) => {}
        }
    }

    result
}

pub fn extract_param_annotations(query: &str) -> BTreeMap<i32, Vec<TsFieldType>> {
    let re = Regex::new(r"@param (\d+): ([^\n]+)").unwrap();
    let captures = re.captures_iter(query);

    captures
        .filter_map(|capture| {
            let index = capture.get(1);
            let types = capture.get(2);
            if index.is_some() && types.is_some() {
                let index = index?.as_str().parse::<i32>().unwrap();
                let types = types?
                    .as_str()
                    .split('|')
                    .map(|t| t.trim())
                    .map(TsFieldType::get_ts_field_from_annotation)
                    .collect::<Vec<TsFieldType>>();

                return Some((index, types.to_owned()));
            }
            None
        })
        .collect()
}
