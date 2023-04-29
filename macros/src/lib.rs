use proc_macro2::{ Literal };
use proc_macro::{ TokenStream };
use quote::{ quote, format_ident };
use syn::fold::{ Fold, fold_signature, fold_fn_arg };
use syn::parse::{ Parse, ParseStream };
use syn::punctuated::Punctuated;
use syn::{ parse_macro_input, ItemFn, Block, Signature, Ident, Visibility, FnArg, Token, Result, Pat };

mod jsx;


struct ArgLen {
    length: Literal
}

impl Parse for ArgLen {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Literal, Token![,]>::parse_terminated(input)?;
        Ok(ArgLen {
            length: vars.first().cloned().unwrap_or_else(|| { Literal::usize_unsuffixed(0) })
        })
    }
}

struct CustomFn {
    name: Option<Ident>,
    args: Vec<FnArg>,
    signature: Option<Signature>,
    visibility: Option<Visibility>,
    blocks: Vec<Block>
}

impl CustomFn {
    fn new() -> Self {
        return CustomFn {
            name: None,
            args: vec![],
            visibility: None,
            signature: None,
            blocks: vec![]
        };
    }
}

impl Fold for CustomFn {
    fn fold_fn_arg(&mut self, f: FnArg) -> FnArg {
        self.args.push(f.clone());
        fold_fn_arg(self, f)
    }
    fn fold_signature(&mut self, sign: Signature) -> Signature {
        self.name = Some(format_ident!("{}", sign.ident));
        self.signature = Some(sign.clone());
        fold_signature(self, sign)
    }

    fn fold_visibility(&mut self, visib: Visibility) -> Visibility {
        self.visibility = Some(visib.clone());
        visib
    }

    fn fold_block(&mut self, block: Block) -> Block {
        self.blocks.push(block.clone());
        block
    }
}

#[proc_macro_attribute]
pub fn custom_log(_: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);
    let mut custom_fn = CustomFn::new();
    custom_fn.fold_item_fn(item_fn);
    let name = custom_fn.name.unwrap();
    let args = custom_fn.args;
    let idents: Vec<Ident> = args.iter().filter_map(|item| { 
        if let FnArg::Typed(p) = item {
            if let Pat::Ident(l) = p.pat.as_ref() {
                return Some(l.ident.clone());
            }
        }
        None
    }).collect();
    let blocks = custom_fn.blocks;
    let visibility = custom_fn.visibility.unwrap();
    let expanded = quote!{ 
        #visibility fn #name(#(#args),*) {
            use utils::flat_print;
            use utils::count_args;
            let count = count_args!(#(#idents),*);
            print!("call({}) {} with ", count, stringify!(#name));
            flat_print!(#(#idents),*);
            #(#blocks)*
        }
     };
    TokenStream::from(expanded)
}