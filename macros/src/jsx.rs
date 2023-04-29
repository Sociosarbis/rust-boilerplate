use proc_macro2::Ident;
use syn::{parse::{ Parse, ParseStream, Result }, Token, Error};

pub struct JSXElement {
  pub tag: Ident
}

impl Parse for JSXElement {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<Token![<]>()?;
      let tag: Ident = input.parse()?;
      if let Ok(_) = input.parse::<Token![/]>() {
        input.parse::<Token![>]>()?;
        Ok(JSXElement {
          tag
        })
      } else {
        input.parse::<Token![>]>()?;
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let ty = input.parse::<Ident>()?;
        if ty.eq(&tag) {
          input.parse::<Token![>]>()?;
          Ok(JSXElement {
            tag
          })
        } else {
          Err(Error::new(ty.span(), "end tag doen't match start tag"))
        }
      }
  }
}