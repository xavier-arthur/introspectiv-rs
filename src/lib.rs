pub mod routes;
pub mod app;
pub mod layout;

pub mod components;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}