//! Switch between Rust editions anywhere in your code.
//!
//! See [the Rust Edition Guide](https://doc.rust-lang.org/edition-guide)
//! for details on all the editions.
//!
//! ## Example
//!
//! ```
//! use edition::*;
//!
//! rust_2015! {
//!     // `async` isn't a keyword yet.
//!     fn async() {}
//! }
//!
//! rust_2018! {
//!     // `async` is a keyword now.
//!     async fn abc() {}
//! }
//!
//! rust_2021! {
//!     // Arrays now implement IntoIter.
//!     let _: i32 = [1, 2, 3].into_iter().next().unwrap();
//! }
//!
//! rust_2024! {
//!     // Box<[T]> now implements IntoIter.
//!     let _: i32 = vec![1, 2, 3].into_boxed_slice().into_iter().next().unwrap();
//! }
//! ```
//!
//! The macros are usable in any context where a macro call is allowed.
//! That is, not only can they wrap items or statements, they can also wrap a type or a (sub)expression:
//!
//! ```rust
//! # use edition::*;
//! use std::string::ToString;
//!
//! # fn main() {
//! let _: rust_2015!(&::ToString) = &rust_2021!([1, 2, 3].into_iter()).next().unwrap();
//! # }
//! ```
//!
//! ## Limitations
//!
//! These macros only work for edition changes that are token based, which are most of them.
//! They do not affect which prelude is imported, what resolver Cargo uses, or how the source code is tokenized.
//!
//! These macros change the context of the token spans to that of a token from the requested edition.
//! This breaks hygiene, meaning that you cannot refer to locals outside the macro invocation:
//!
//! ```compile_fail,E0425
//! # use edition::*;
//! let a = 123;
//!
//! rust_2015! {
//!     let _ = a; // Error: cannot find value `a` in this scope
//! }
//! ```

/// Switch to Rust 2015.
///
/// ```
/// # use edition::*;
/// rust_2015! {
///     fn async() {} // `async` isn't a keyword yet.
/// }
/// ```
#[cfg(feature = "rust-2015")]
pub use rust_2015::rust_2015;

/// Switch to Rust 2018.
///
/// ```
/// # use edition::*;
/// rust_2018! {
///     async fn abc() {} // `async` is a keyword now.
/// }
///
/// let _: rust_2018!(&ToString) = &123; // `dyn` isn't required yet.
/// ```
#[cfg(feature = "rust-2018")]
pub use rust_2018::rust_2018;

/// Switch to Rust 2021.
///
/// ```
/// # use edition::*;
/// rust_2021! {
///     // Arrays now implement IntoIter.
///     let _: i32 = [1, 2, 3].into_iter().next().unwrap();
///
///     // But Box<[T]> doesn't yet. (Note the &i32 instead of i32.)
///     let _: &i32 = vec![1, 2, 3].into_boxed_slice().into_iter().next().unwrap();
/// }
/// ```
#[cfg(feature = "rust-2021")]
pub use rust_2021::rust_2021;

/// Switch to Rust 2024.
///
/// ```
/// # use edition::*;
/// rust_2024! {
///     // Box<[T]> now implements IntoIter.
///     let _: i32 = vec![1, 2, 3].into_boxed_slice().into_iter().next().unwrap();
/// }
/// ```
#[cfg(feature = "rust-2024")]
pub use rust_2024::rust_2024;
