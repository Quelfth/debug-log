
#[macro_export]
macro_rules! log {
    ($($input:tt)*) => {
        println!("[{}:{}:{}] {}", file!(), line!(), column!(), format!($($input)*))
    };
}