use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsGeneratorError {
  #[error("[E001] Unable to infer an appropriate name for the query - query: `{0}`")]
  EmptyQueryNameFromVarDecl(String),
  #[error("[E002] Failed to fetch query name from the annotation - query: `{0}`")]
  EmptyQueryNameFromAnnotation(String),
  #[error("[E003] Missing alias when handling functions - query: `{0}`")]
  MissingAliasForFunctions(String),
  #[allow(dead_code)]
  #[error("[E004] Invalid Typescript file path - file path: `{0}`")]
  InvalidTypescriptFilePath(PathBuf),
  #[error("[E005] Failed to handle a wildcard statement without target tables in `FROM` statement - query: `{0}`")]
  WildcardStatementWithoutTargetTables(String),
  #[error("[E006] Failed to handle a wildcard statement as it reached a dead-end expression - query: `{0}`")]
  WildcardStatementDeadendExpression(String),
  #[error("[E007] Unsupported table with joins statement detected - query: `{0}`")]
  WildcardStatementUnsupportedTableExpr(String),
  #[error("[E008] The query contains unknown placeholder parameter symbol - query: `{0}`")]
  UnknownPlaceholder(String),
  #[error("[E009] When translating a function in a SELECT clause, you must provide an alias - query : `{0}`")]
  FunctionWithoutAliasInSelectClause(String),
  #[error("[E010] Unknown function detected while processing a SELECT clause - query: `{0}`")]
  #[allow(dead_code)]
  FunctionUnknown(String),
  // Errors while handling FROM statement
  #[error("[E012] Failed to handle a from statement without the `FROM` keyword - query: `{0}`")]
  FromWithoutKeyword(String),
  #[error("[E013] Failed to handle a table factor of a FROM statement: table factor: `{0}`")]
  TableFactorWhileProcessingTableWithJoins(String),
  #[error("[E014] Failed to find a table name from a FROM statement: statement: `{0}`")]
  UnknownErrorWhileProcessingTableWithJoins(String),
  #[error("[E015] Table expressions are not supported in INSERT statements - query: `{0}`")]
  TableExpressionInInsertStatement(String),
  #[error("[E016] Column '{column}' not found in table '{table}'. Available columns: {available_columns}")]
  ColumnNotFoundInTable {
    column: String,
    table: String,
    available_columns: String,
  },
  #[error("[E017] Failed to process INSERT statement: {reason}. Query: `{query}`")]
  InsertStatementProcessingFailed { reason: String, query: String },
  #[error("[E018] Failed to process UPDATE statement: {reason}. Query: `{query}`")]
  UpdateStatementProcessingFailed { reason: String, query: String },
  #[error("[E018] Table '{table}' not found in database schema. Check that the table exists and is accessible.")]
  TableNotFoundInSchema { table: String },
  #[error("[E019] Failed to infer table name while processing WHERE clause. Query: `{query}`")]
  TableNameInferenceFailedInWhere { query: String },
  #[error("Unknown error: `{0}`")]
  Unknown(String),
}
