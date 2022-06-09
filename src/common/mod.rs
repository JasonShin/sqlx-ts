extern crate core;

use swc_common::MultiSpan;

pub mod cli;
pub mod config;
pub mod dotenv;
pub mod types;

// Source Parser
#[derive(Debug)]
pub struct SQL {
    pub query: String,
    pub span: MultiSpan,
}
