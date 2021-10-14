use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse::{Parse, ParseStream}};

pub(crate) struct StringInput {
    pub string: String,
}

impl Parse for StringInput {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        if stream.is_empty() {
            panic!("Need a string something to parse");
        }

        let str: LitStr =  stream.parse().unwrap();

        Ok(Self {
            string:str.value(),
        })
    }
}

#[proc_macro]
pub fn macro_impl(input: TokenStream) -> TokenStream {
    let string = syn::parse_macro_input!(input as StringInput);
    let string = string.string;
    let tokens = quote! {
        #string
    };

    tokens.into()
}
