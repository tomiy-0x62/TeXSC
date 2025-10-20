#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            let conf = $crate::CONFIG.read().expect("couldn't read CONFIG");
            if cfg!(debug_assertions) || conf.debug {
                eprint!($($arg)*);
            }
        }
    }
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {
        {
            let conf = $crate::CONFIG.read().expect("couldn't read CONFIG");
            if cfg!(debug_assertions) || conf.debug {
                eprintln!($($arg)*);
            }
        }
    }
}

#[macro_export]
macro_rules! eprintlnc {
    ($e:expr) => {
        eprintln!("{}: {}", "Error".red(), $e)
    };
}
