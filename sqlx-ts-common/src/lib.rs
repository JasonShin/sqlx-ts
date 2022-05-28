#[macro_use]
extern crate serde_derive;
extern crate core;

use swc_common::MultiSpan;

pub mod cli;
pub mod config;

// Source Parser
#[derive(Debug)]
pub struct SQL {
    pub query: String,
    pub span: MultiSpan,
}
