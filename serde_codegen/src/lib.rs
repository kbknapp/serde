#![cfg_attr(not(feature = "syntex"), feature(custom_derive, rustc_private, unboxed_closures))]
#![cfg_attr(not(feature = "syntex"), plugin(quasi_macros))]

extern crate aster;
extern crate quasi;

#[cfg(feature = "syntex")]
extern crate syntex;

#[cfg(feature = "syntex")]
extern crate syntex_syntax as syntax;

#[cfg(not(feature = "syntex"))]
extern crate syntax;

#[cfg(feature = "syntex")]
include!(concat!(env!("OUT_DIR"), "lib.rs"));

#[cfg(not(feature = "syntex"))]
include!("lib.rs.in"));

#[cfg(feature = "syntex")]
pub fn register(reg: &mut syntex::Registry) {
    reg.register_decorator("derive_Serialize", ser::expand_derive_serialize);
    reg.register_decorator("derive_Deserialize", ser::expand_derive_deserialize);
}
