#![allow(unknown_lints, unexpected_cfgs)]
#![allow(clippy::needless_doctest_main)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

mod cli;

use cli::setup;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Parser};
use syn::{braced, Attribute, Ident, Path, Signature, Visibility};

// This `extern` is required for older `rustc` versions but newer `rustc`
// versions warn about the unused `extern crate`.
#[allow(unused_extern_crates)]
extern crate proc_macro;

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    setup();
    validate(item)
}

pub(crate) fn validate(item: TokenStream) -> TokenStream {
    syn::parse2(item.clone()).unwrap();
    item
}

#[derive(Debug)]
struct ItermFn {
    sig: Signature,
}

impl Parse for ItemFn {
    #[inline]
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let outer_attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;

        if sig.ident != "main" {
            return Err("macro should be aplied only to the main function!");
        }

        Ok(Self { sig })
    }
}
