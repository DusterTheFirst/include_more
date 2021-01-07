use std::{fs, path::{Path, PathBuf}};

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

pub fn include(file: LitStr) -> syn::Result<TokenStream> {
    // Get the full path to the file
    // FIXME: once https://github.com/rust-lang/rust/issues/54725 gets resolved
    let input_file_path = PathBuf::from(format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("environment variable `CARGO_MANIFEST_DIR` must be set"),
            file.value()
    ));

    // Enforce the existence of the file
    if !input_file_path.exists() {
        return Err(Error::new(
            file.span(),
            format!("File `{}` does not exist", csv_path.to_string_lossy()),
        ));
    }

    let input_file_data = fs::read_to_string(input_file_path).map_err(|e| {

    });
    // let file_str = input_file.to_string_lossy();

    quote! {
        #file_str
    }
}
