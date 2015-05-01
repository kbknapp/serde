extern crate syntex;
extern crate serde_codegen;

use std::env;
use std::path::Path;


fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut registry = syntex::Registry::new();
    serde_codegen::register(&mut registry);

    let src = Path::new("tests/test.rs.in");
    let dst = Path::new(&out_dir).join("test.rs");

    registry.expand("", &src, &dst).unwrap();

    let src = Path::new("benches/bench.rs.in");
    let dst = Path::new(&out_dir).join("bench.rs");

    registry.expand("", &src, &dst).unwrap();
}
