//! # Differences from `std::include_*` macros
//!
//! Unlike the `include_str`, `include_bytes`, and `include` macros provided by
//! the standard library, these macro resolves the provided paths relative to the
//! Cargo.toml file of the package it is used in rather than the file it is used.
//! This difference is due to the lack of a stable api to get the file that a
//! macro was called in. The issue https://github.com/rust-lang/rust/issues/54725
//! tracks the addition of such apis, but until those are stabilized, there is no
//! way to make the file resolution relative to the current file.

#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

mod sass;
mod ts;

/// Includes a UTF-8 encoded scss/sass file as its compiled css.
///
/// The file is located relative to the package root . The provided path is
/// interpreted in a platform-specific way at compile time. So, for instance, an
/// invocation with a Windows path containing backslashes `\` would not compile
/// correctly on Unix.
///
/// This macro will yield an expression of type `&'static str` which is the
/// compiled css produced from the file.
#[proc_macro]
pub fn include_sass(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    sass::include(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Includes a UTF-8 encoded typescript file as its compiled js.
///
/// The file is located relative to the package root . The provided path is
/// interpreted in a platform-specific way at compile time. So, for instance, an
/// invocation with a Windows path containing backslashes `\` would not compile
/// correctly on Unix.
///
/// This macro will yield an expression of type `&'static str` which is the
/// compiled css produced from the file.
///
/// # Warning
/// This macro does not use the official typescript compiler and so is unable to
/// do any type checking, solely transpilation into js. It is recommended to also
/// run the official typescript compiler along with this macro to ensure the
/// included typescript is valid
#[proc_macro]
pub fn include_ts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    ts::include(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
