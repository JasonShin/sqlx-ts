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

        if let (Some(name), Some(types)) = (name, types) {
            let name = name.as_str().to_string();
            let types = types
                .as_str()
                .split('|')
                .map(|t| t.trim())
                .map(TsFieldType::get_ts_field_from_annotation)
                .collect::<Vec<TsFieldType>>();

            result.insert(name, types);
        }
    }

    result
}

pub fn extract_param_annotations(query: &str) -> BTreeMap<usize, TsFieldType> {
    let re = Regex::new(r"@param (\d+): ([^\n]+)").unwrap();
    let captures = re.captures_iter(query);

    captures
        .filter_map(|capture| {
            let index = capture.get(1);
            let types = capture.get(2);
            if index.is_some() && types.is_some() {
                let index = index?.as_str().parse::<usize>().unwrap();
                let types = types?.as_str();
                let types = TsFieldType::get_ts_field_from_annotation(types);
                return Some((index, types));
            }
            None
        })
        .collect()
}
