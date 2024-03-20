#[macro_export]
macro_rules! print_message {
  ($($arg:tt)*) => {{
    util::show_game_message(format!($($arg)*))
}}
}
