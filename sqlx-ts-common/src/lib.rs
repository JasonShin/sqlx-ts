use swc_common::MultiSpan;

pub mod cli;
pub mod config;

// Source Parser
pub struct SQL {
    pub query: String,
    pub span: MultiSpan,
}
