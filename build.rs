use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rerun-if-changed=memory.x");
}
