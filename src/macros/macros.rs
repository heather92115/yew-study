
#[macro_export]
macro_rules! debug_object {

    ($expr1:expr, $expr2:expr) => {
        web_sys::console::log_1(&format!("[{}:{}] {:?} {:?}", file!(), line!(), $expr1, $expr2).into());
    };
    ($expr:expr) => {
        web_sys::console::log_1(&format!("[{}:{}] {:?}", file!(), line!(), $expr).into());
    };
}