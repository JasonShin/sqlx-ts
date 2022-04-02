use clap::ArgEnum;

#[derive(ArgEnum, Debug, Clone)]
pub enum JsExtension {
    Ts,
    Js,
}

impl ToString for JsExtension {
    fn to_string(&self) -> String {
        match self {
            | JsExtension::Ts => ".ts".to_string(),
            | JsExtension::Js => ".js".to_string(),
        }
    }
}
