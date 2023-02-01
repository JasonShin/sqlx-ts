extern crate core;

use swc_common::MultiSpan;

pub mod cli;
pub mod config;
pub mod dotenv;
pub mod types;
pub mod lazy;

// Source Parser
#[derive(Debug, Clone)]
pub struct SQL {
    /// Note that not all sql`` statements belong to a variable expression, therefore we must store it as an option
    pub var_decl_name: Option<String>,
    pub query: String,
    pub span: MultiSpan,
}
