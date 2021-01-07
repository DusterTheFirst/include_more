use std::{ops::Deref, path::{Path, PathBuf}};

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Token,
};

pub struct WrappedFile(PathBuf);

impl Parse for WrappedFile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut inputs = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?.into_iter();

        let filename = inputs
            .next()
            .ok_or_else(|| syn::Error::new(input.span(), "expected 2 arguments, found 0"))?;

        let called_file = inputs
            .next()
            .ok_or_else(|| syn::Error::new(input.span(), "expected 2 arguments, found 1"))?;

        if let Some(excess) = inputs.next() {
            return Err(syn::Error::new(
                excess.span(),
                "unexpected arguments. macro takes 2 arguments",
            ));
        }

        let file = [
            PathBuf::from(called_file.value()).parent().unwrap_or_else(|| Path::new("/")),
            Path::new(&filename.value())
        ].iter().collect::<PathBuf>();

        Ok(Self(file))
    }
}

impl Deref for WrappedFile {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
