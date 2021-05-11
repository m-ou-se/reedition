extern crate reedition;

#[macro_export]
macro_rules! rust_2015 {
    ($($t:tt)*) => {
        $crate::__export::reedition! {
            hello
            ($($t)*)
        }
    }
}

#[doc(hidden)]
pub mod __export {
    pub use ::reedition::reedition;
}
