#[macro_use]
extern crate rust_2018;

#[macro_use]
extern crate rust_2021;

rust_2018! {
    async fn hey() {}
}

fn main() {
    let async = 123;
    let _ = async;
    let _ = hey();
    let _: i32 = rust_2021!([1, 2, 3].into_iter().next().unwrap());
}
