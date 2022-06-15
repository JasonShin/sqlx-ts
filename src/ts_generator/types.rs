use std::collections::HashMap;
use std::fmt::{self};

pub enum TsDataType {
    Boolean,
    Number,
    String,
}

impl fmt::Display for TsDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsDataType::Boolean => write!(f, "{}", "boolean".to_string()),
            TsDataType::Number => write!(f, "{}", "number".to_string()),
            TsDataType::String => write!(f, "{}", "string".to_string()),
        }
    }
}

pub struct TsQuery {
    pub name: String,
    pub params: HashMap<String, TsDataType>,
    pub result: HashMap<String, TsDataType>,
}

impl TsQuery {
    fn fmt_attributes_map(
        &self,
        f: &mut fmt::Formatter<'_>,
        attrs_map: &HashMap<String, TsDataType>,
    ) -> String {
        let result: Vec<String> = attrs_map
            .into_iter()
            .map(|(name, data_type)| format!("{name}: {data_type};"))
            .collect();

        format!("{}", result.join("\n").to_string())
    }
}

impl fmt::Display for TsQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let params_str = self.fmt_attributes_map(f, &self.params);
        let result_str = self.fmt_attributes_map(f, &self.result);

        let params = format!(
            r"
            export interface I{name}Params {{
                {params_str}
            }}
        "
        );

        let result = format!(
            r"
            export interface I{name}Result {{
                {result_str}
            }}
        "
        );

        let query = format!(
            r"
            export interface I{name}Query {{
                params: I{name}Params;
                result: I{name}Result;
            }}
        "
        );

        let final_code = format!(
            r"
            {params};
            {result};
            {query};"
        );

        writeln!(f, "{}", final_code)
    }
}
