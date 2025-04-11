#[macro_export]
macro_rules! rust_2024 {
    ($($t:tt)*) => {
        $crate::__export::reedition! {
            hello
            ($($t)*)
        }
    }
}

#[doc(hidden)]
pub mod __export {
    pub use reedition::reedition;
}
