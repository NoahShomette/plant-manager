use std::env;

fn main() {
    dotenvy::dotenv().unwrap();
    println!(
        "cargo::rustc-env=SERVER_ADDR={}",
        env::var("SERVER_ADDR").unwrap()
    );
}
