//! See [the `edition` crate](https://docs.rs/edition/).

#[macro_export]
macro_rules! rust_2021 {
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
