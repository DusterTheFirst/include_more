#![forbid(unsafe_code)]

use input::WrappedFile;
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

mod input;
mod sass;

#[proc_macro]
pub fn include_sass(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    sass::include(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
