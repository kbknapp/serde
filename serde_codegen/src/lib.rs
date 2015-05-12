#![cfg_attr(feature = "nightly", feature(rustc_private, plugin))]
#![cfg_attr(feature = "nightly", plugin(quasi_macros))]

extern crate quasi;

#[cfg(feature = "nightly")]
pub mod syntax {
    extern crate syntax;
    extern crate rustc;

    mod inner {
        extern crate aster_syntax as aster;
        pub use super::syntax;

        include!("lib.rs.in");
    }

    pub fn register(reg: &mut rustc::plugin::Registry) {
        reg.register_syntax_extension(
            syntax::parse::token::intern("derive_Serialize"),
            syntax::ext::base::Decorator(Box::new(inner::ser::expand_derive_serialize)));

        reg.register_syntax_extension(
            syntax::parse::token::intern("derive_Deserialize"),
            syntax::ext::base::Decorator(Box::new(inner::de::expand_derive_deserialize)));
    }
}

#[cfg(feature = "syntex")]
pub mod syntex {
    extern crate syntex;

    mod inner {
        extern crate aster_syntex as aster;
        extern crate syntex_syntax as syntax;

        include!(concat!(env!("OUT_DIR"), "/lib.rs"));
    }

    pub fn register(reg: &mut syntex::Registry) {
        reg.register_decorator("derive_Serialize", inner::ser::expand_derive_serialize);
        reg.register_decorator("derive_Deserialize", inner::de::expand_derive_deserialize);
    }
}
