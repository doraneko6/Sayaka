use std::env;

use once_cell::sync::Lazy;

pub const SAYAKA_DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("SAYAKA_DEBUG") {
        Ok(value) => {
            if value.parse::<u8>().unwrap() > 0 { true } else { false }
        }
        Err(_) => false
    }
});

pub const SAYAKA_NO_COLOR: Lazy<bool> = Lazy::new(|| {
    match env::var("SAYAKA_NO_COLOR") {
        Ok(value) => {
            if value.parse::<u8>().unwrap() > 0 { true } else { false }
        }
        Err(_) => false
    }
});

// For external package macro
#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        if *sayaka::debugger::SAYAKA_DEBUG {
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
            if *sayaka::debugger::SAYAKA_NO_COLOR {
                eprint!("[{}][{}][{}]", "DEBUG", timestamp, format!("{}:{}", file!(), line!()));
            } else {
                eprint!("[{}][{}][{}]", "DEBUG".green(), timestamp.yellow(), format!("{}:{}", file!(), line!()).cyan());
            }
            eprint!(" ");
            eprintln!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_fn {
    ($($expression:expr), *) => {
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        if *sayaka::debugger::SAYAKA_DEBUG {
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();if *sayaka::debugger::SAYAKA_NO_COLOR {
                eprint!("[{}][{}][{}]", "DEBUG", timestamp, format!("{}:{}", file!(), line!()));
            } else {
                eprint!("[{}][{}][{}]", "DEBUG".green(), timestamp.yellow(), format!("{}:{}", file!(), line!()).cyan());
            }
            eprint!(" Calling {}(),", name.strip_suffix("::f").unwrap());
            $(
                {
                    eprint!(" {:?} = {:?}", stringify!($expression), &$expression);
                }
            )*
            eprintln!();
        }
    };
}

#[macro_export]
macro_rules! debug_var {
    ($($expression:expr), *) => (
        if *sayaka::debugger::SAYAKA_DEBUG {
            $(
                {
                    let now = chrono::Local::now();
                    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
                    if *sayaka::debugger::SAYAKA_NO_COLOR {
                        eprint!("[{}][{}][{}]", "DEBUG", timestamp, format!("{}:{}", file!(), line!()));
                    } else {
                        eprint!("[{}][{}][{}]", "DEBUG".green(), timestamp.yellow(), format!("{}:{}", file!(), line!()).cyan());
                    }
                    eprint!(" ");
                    eprint!("{:?} = {:#?}",stringify!($expression),&$expression);
                    eprintln!();
                }
            )*
        }
    )
}

// For local use
#[macro_export]
macro_rules! debug_fn_inline {
    ($($expression:expr), *) => {
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        if *crate::debugger::SAYAKA_DEBUG {
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
            if *crate::debugger::SAYAKA_NO_COLOR {
                eprint!("[{}][{}][{}]", "DEBUG", timestamp, format!("{}:{}", file!(), line!()));
            } else {
                eprint!("[{}][{}][{}]", "DEBUG".green(), timestamp.yellow(), format!("{}:{}", file!(), line!()).cyan());
            }
            eprint!(" Calling {}(),", name.strip_suffix("::f").unwrap());
            $(
                {
                    eprint!(" {:?} = {:?}", stringify!($expression), &$expression);
                }
            )*
            eprintln!();
        }
    };
}

#[macro_export]
macro_rules! debugln_inline {
    ($($arg:tt)*) => {{
        if *crate::debugger::SAYAKA_DEBUG {
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
            if *crate::debugger::SAYAKA_NO_COLOR {
                eprint!("[{}][{}][{}]", "DEBUG", timestamp, format!("{}:{}", file!(), line!()));
            } else {
                eprint!("[{}][{}][{}]", "DEBUG".green(), timestamp.yellow(), format!("{}:{}", file!(), line!()).cyan());
            }
            eprint!(" ");
            eprintln!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_var_inline {
    ($($expression:expr), *) => (
        if *crate::debugger::SAYAKA_DEBUG {
            $(
                {
                    let now = chrono::Local::now();
                    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
                    if *crate::debugger::SAYAKA_NO_COLOR {
                        eprint!("[{}][{}][{}]", "DEBUG", timestamp, format!("{}:{}", file!(), line!()));
                    } else {
                        eprint!("[{}][{}][{}]", "DEBUG".green(), timestamp.yellow(), format!("{}:{}", file!(), line!()).cyan());
                    }
                    eprint!(" ");
                    eprint!("{:?} = {:#?}",stringify!($expression),&$expression);
                    eprintln!();
                }
            )*
        }
    )
}