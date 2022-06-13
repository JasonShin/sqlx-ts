use std::collections::HashMap;
use std::fmt;

pub enum TsDataType {
    Boolean,
    Number,
    String,
}

pub type TsDataMap = HashMap<String, TsDataType>;

impl fmt::Display for TsDataMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let z = self;
        // generate lines of name: datatype
    }
}

pub struct TsQuery {
    pub name: String,
    pub params: HashMap<String, TsDataType>,
    pub result: HashMap<String, TsDataType>,
}

impl fmt::Display for TsQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name;
        
        let params = format!(r"
export interface I{name}Params
        ");

        let final_code = format!("{}; {}; {};");

        writeln!(f, "{}", final_code)
    }
}
