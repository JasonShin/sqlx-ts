use clap::ArgEnum;

#[derive(ArgEnum, Debug, Clone)]
pub enum JsExtension {
    Ts,
    Js,
}

#[derive(ArgEnum, Debug, Clone)]
pub enum DatabaseType {
    Postgres,
    Mysql,
}
