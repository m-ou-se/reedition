# Edition!

Switch between Rust editions anywhere in your code.

See [the Rust Edition Guide](https://doc.rust-lang.org/edition-guide) for details on all the editions.

## Example

```rust
use edition::*;

rust_2015! {
    // `async` isn't a keyword yet.
    fn async() {}
}

rust_2018! {
    // `async` is a keyword now.
    async fn abc() {}
}

rust_2021! {
    // Arrays now implement IntoIter.
    let _: i32 = [1, 2, 3].into_iter().next().unwrap();
}

rust_2024! {
    // Box<[T]> now implements IntoIter.
    let _: i32 = vec![1, 2, 3].into_boxed_slice().into_iter().next().unwrap();
}
```

The macros are usable in any context where a macro call is allowed.
That is, not only can they wrap items or statements, they can also wrap a type or a (sub)expression:

```rust
use std::string::ToString;

let _: rust_2015!(&::ToString) = &rust_2021!([1, 2, 3].into_iter()).next().unwrap();
```

## Limitations

These macros only work for edition changes that are token based, which are most of them.
They do not affect which prelude is imported, what resolver Cargo uses, or how the source code is tokenized.

These macros change the context of the token spans to that of a token from the requested edition.
This breaks hygiene, meaning that you cannot refer to locals outside the macro invocation:

```rust
let a = 123;

rust_2015! {
    let _ = a; // Error: cannot find value `a` in this scope
}
```
