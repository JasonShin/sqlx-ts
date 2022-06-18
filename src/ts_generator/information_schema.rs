use std::collections::HashMap;

pub enum TsFieldType {
    String,
    Number,
    Boolean,
    Object,
    Any,
}

pub struct Field {
    field_type: TsFieldType,
    is_nullable: bool,
}

pub type Fields = HashMap<String, Field>;

trait DBSchema {
    fn fetch_table_schema(&self, table_name: String);
    fn get_fields(&self, table_name: String) -> Fields;
}

pub struct MySQLSchema {
    pub tables: HashMap<String, Fields>,
}

impl DBSchema for MySQLSchema {
    fn fetch_table_schema(&self, table_name: String) {
        
    }

    fn get_fields(&self, table_name: String) -> Fields {
        
    }
}

pub struct PostgresSchema {
    pub tables: HashMap<String, Fields>,
}


