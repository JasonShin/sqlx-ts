use sqlparser::ast::{Query, Statement};

#[derive(Debug)]
pub struct SqlQuery {
    /// The field stores the parent level SQL statements, mostly used for wildcard queries
    pub sql_statement: Statement,
    ///
    pub query: Box<Query>,
}

impl SqlQuery {
    pub fn set_sql_statement(&mut self, sql_statement: &Statement) {
        self.sql_statement = sql_statement.to_owned()
    }

    pub fn set_query(&mut self, query: &Box<Query>) {
        self.query = query.to_owned();
    }
}
