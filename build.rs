use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rustc-link-search=native=link-deps/{}",
        env::var("TARGET").expect("Cargo should provide the TARGET env variable."),
    );
}
