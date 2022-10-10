use sqlparser::ast::{TableWithJoins, TableFactor};

pub fn translate_table_with_joins(table_with_joins: &Vec<TableWithJoins>, potential_table_name: Option<String>) {
    let default_table_name = table_with_joins
    .get(0)
    .map(|x| {
        match &x.relation {
            TableFactor::Table { name, alias, args, with_hints } => {
                alias.clone()
                    .and_then(|x| {
                        Some(x.name.value)
                    })
                    .or_else(|| {
                        // Relation must have the first item which is the table name itself
                        Some(name.to_owned().0[0].to_string())
                    })
            },
            _ => None,
        }
    });
}
