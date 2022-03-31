use swc_common::{MultiSpan, Span};

pub struct SQL {
    pub query: String,
    pub span: MultiSpan,
}
