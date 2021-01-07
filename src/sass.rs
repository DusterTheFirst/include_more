use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;
use rsass::output::{Format, Style};
use syn::{Error, LitStr, Result};

pub fn include(file: LitStr) -> Result<TokenStream> {
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
            format!("file {:?} does not exist", input_file_path),
        ));
    }

    let compiled_css = rsass::compile_scss_file(
        &input_file_path,
        Format {
            style: Style::Compressed,
            precision: 5,
        },
    )
    .map_err(|e| {
        Error::new(
            file.span(),
            format!("failed to parse {:?}: {:?}", input_file_path, e),
        )
    })?;

    let compiled_css = String::from_utf8_lossy(&compiled_css);
    let input_file_path = input_file_path.to_string_lossy();

    Ok(quote! {
        {
            include_bytes!(#input_file_path);

            #compiled_css
        }
    })
}
