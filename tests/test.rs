#![feature(test)]
#![cfg_attr(not(feature = "syntex"), feature(custom_attribute, custom_derive, plugin))]
#![cfg_attr(not(feature = "syntex"), plugin(serde_macros))]

extern crate test;
extern crate serde;

#![cfg(feature = "syntex")]
include!(concat!(env!("OUT_DIR"), "test.rs"));

#![cfg(not(feature = "syntex"))]
include!("test.rs.in");
