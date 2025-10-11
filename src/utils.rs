
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        print!("\x1b[96mDEBUG:\x1b[0m ");
        println!("\x1b[93m{}\x1b[0m", format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        print!("\x1b[92mINFO:\x1b[0m ");
        println!("\x1b[97m{}\x1b[0m", format!($($arg)*));
    }};
}
