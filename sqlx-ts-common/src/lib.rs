use swc_common::MultiSpan;

pub mod config;

pub struct SQL {
    pub query: String,
    pub span: MultiSpan,
}
