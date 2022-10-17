#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        let conf = $crate::config::read_config().unwrap();
        if cfg!(debug_assertions) || conf.debug {
            eprint!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {
        let conf = $crate::config::read_config().unwrap();
        if cfg!(debug_assertions) || conf.debug {
            eprintln!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! eprintlnc {
    ($e:expr) => {
        eprintln!("{}: {}", "Error".red(), $e)
    };
}
