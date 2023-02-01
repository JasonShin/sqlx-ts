use clap::Parser;
use lazy_static::lazy_static;
use crate::common::config::Config;
use crate::common::cli::Cli;

/// The file contains all implicitly dependent variables or state that files need for the logic
/// We have a lot of states that we need to drill down into each methods
lazy_static! {
    pub static ref SOME_INT: i32 = 5;

    pub static ref CLI_ARGS: Cli = Cli::parse();
    pub static ref CONFIG: Config =  Config::new(CLI_ARGS.to_owned());
}


