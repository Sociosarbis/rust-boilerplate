use proc_macro::TokenStream;
use proc_macro2::{Ident};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::{ Parse, ParseStream, Result }, Token, Error, Expr, braced, parse_macro_input };

pub struct Prop {
  pub name: Ident,
  pub value: Vec<Expr>
}

pub struct JSXElement {
  pub tag: Ident,
  pub props: Vec<Prop>,
  pub children: Vec<JSXElement>
}

impl Parse for JSXElement {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<Token![<]>()?;
      let tag: Ident = input.parse()?;
      let mut props = vec![];
      while !(input.peek(Token![/]) || input.peek(Token![>])) {
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let content;
        braced!(content in input);
        let mut stmts = vec![];
        while !content.is_empty() {
          stmts.push(content.parse::<Expr>()?);
        }
        props.push(Prop {
          name,
          value: stmts
        }); 
      }
      if let Ok(_) = input.parse::<Token![/]>() {
        input.parse::<Token![>]>()?;
        Ok(JSXElement {
          tag,
          props,
          children: vec![]
        })
      } else {
        input.parse::<Token![>]>()?;
        let mut children = vec![];
        while !(input.peek(Token![<]) && input.peek2(Token![/])) {
          children.push(JSXElement::parse(input)?);
        }
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let ty = input.parse::<Ident>()?;
        if ty.eq(&tag) {
          input.parse::<Token![>]>()?;
          Ok(JSXElement {
            tag,
            props,
            children
          })
        } else {
          Err(Error::new(ty.span(), "end tag doesn't match start tag"))
        }
      }
  }
}

impl ToTokens for Prop {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = self.name.to_string();
    let value = &self.value;
    tokens.append_all(quote! {
      format!("{}:{}", #name, {#(#value)*})
    })
  }
}

impl ToTokens for JSXElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let tag_name = self.tag.to_string();
      let props = &self.props;
      let children = &self.children;
      tokens.append_all(quote! {
        format!("createElement('{}', {{{}}}, [{}])", #tag_name, (vec![#(#props),*] as Vec<String>).join(","), (vec![#(#children),*] as Vec<String>).join(","))
      })
    }
}

pub fn parse_jsx_impl(input: TokenStream) -> TokenStream {
  let el = parse_macro_input!(input as JSXElement);
  quote! {
    println!("{}", #el);
  }.into()
}