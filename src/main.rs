#![cfg_attr(feature = "strict-build", deny(warnings))]

#![cfg_attr(
feature = "cargo-clippy",
deny(result_unwrap_used, panicking_unwrap, option_unwrap_used)
)]

fn main() {
    println!("Hello, world!");
    res().unwrap();
}

fn res() -> Result<(), ()> {
    Err(())
}
