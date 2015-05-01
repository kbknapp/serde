#![feature(custom_derive, rustc_private, unboxed_closures)]
#![plugin(quasi_macros)]

extern crate aster;
extern crate quasi;
extern crate syntax;

mod ser;
mod de;
mod field;
