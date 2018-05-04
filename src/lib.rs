#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod serde_utils;

pub mod v2_5;
pub mod native;

pub use v2_5 as current;

#[cfg(test)]
mod tests {
    use super::current::*;

    #[test]
    fn test_mods() {
        let _c = Category::Automotive(Automotive::Automotive);
    }
}