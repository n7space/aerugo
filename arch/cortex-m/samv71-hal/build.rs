use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}
