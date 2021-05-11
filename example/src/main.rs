#[macro_use]
extern crate rust_2018;

fn main() {
    let async = 123;
    rust_2018! {
        async fn hey() {}
    }
    let _ = async;
    let _ = hey();
}
