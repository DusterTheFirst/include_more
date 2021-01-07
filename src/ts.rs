use std::{path::PathBuf, sync::Arc};

use proc_macro2::TokenStream;
use quote::quote;
use swc::{
    config::{Config, Options},
    Compiler,
};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
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

    let cm: Arc<SourceMap> = Default::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let compiler = Compiler::new(cm.clone(), handler);

    let fm = cm.load_file(&input_file_path).map_err(|e| {
        Error::new(
            file.span(),
            format!("failed to load file {:?}: {:?}", input_file_path, e),
        )
    })?;

    let compiled_js = compiler
        .process_js_file(
            fm,
            &Options {
                config: Some(Config {
                    minify: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .map_err(|e| {
            Error::new(
                file.span(),
                format!("failed to parse {:?}: {:?}", input_file_path, e),
            )
        })?;

    let compiled_js = compiled_js.code;
    let input_file_path = input_file_path.to_string_lossy();

    Ok(quote! {
        {
            include_bytes!(#input_file_path);

            #compiled_js
        }
    })
}
