#![cfg(feature = "syntex")]
include!(concat!(env!("OUT_DIR"), "bench.rs"));

#![cfg(not(feature = "syntex"))]
include!("bench.rs.in");
