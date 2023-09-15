macro_rules! info {
    ($arg:tt) => ({
        use colored::*;
        let level = "[INFO]".cyan();
        println!("{level} {}", $arg)
    });
    ($arg:tt, $($arg2:tt)*) => ({
        use colored::*;
        let level = "[INFO]".cyan();
        println!("{level} {}", format!($arg, $($arg2)*))
    });
}

pub(crate) use info;

macro_rules! warning {
    ($arg:tt) => ({
        use colored::*;
        let level = "[WARN]".yellow();
        println!("{level} {}", $arg)
    });
    ($arg:tt, $($arg2:tt)*) => ({
        use colored::*;
        let level = "[WARN]".yellow();
        println!("{level} {}", format!($arg, $($arg2)*))
    });
}

pub(crate) use warning;

macro_rules! error {
    ($arg:tt) => ({
        use colored::*;
        let level = "[ERROR]".red();
        let message = $arg;
        eprintln!("{level} {message}")
    });
    ($arg:tt, $($arg2:tt)*) => ({
        use colored::*;
        let level = "[ERROR]".red();
        let message = format!("{}", format!($arg, $($arg2)*));
        eprintln!("{level} {message}")
    });
}

pub(crate) use error;
