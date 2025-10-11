// ANSI color codes
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

// Regular colors
pub const RED: &str = "\x1b[91m";
pub const GREEN: &str = "\x1b[92m";
pub const YELLOW: &str = "\x1b[93m";
pub const BLUE: &str = "\x1b[94m";
pub const MAGENTA: &str = "\x1b[95m";
pub const CYAN: &str = "\x1b[96m";
pub const WHITE: &str = "\x1b[97m";

// Bright colors (alternative)
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        use crate::utils::{CYAN, YELLOW, RESET};
        print!("{}DEBUG:{} ", CYAN, RESET);
        println!("{}{}{}", YELLOW, format!($($arg)*), RESET);
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use crate::utils::{GREEN, WHITE, RESET};
        print!("{}INFO:{} ", GREEN, RESET);
        println!("{}{}{}", WHITE, format!($($arg)*), RESET);
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use crate::utils::{RED, WHITE, RESET};
        print!("{}ERROR:{} ", RED, RESET);
        println!("{}{}{}", WHITE, format!($($arg)*), RESET);
    }};
}