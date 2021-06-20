use self::pipeline::Pipeline;
use proc_macro::TokenStream;
#[cfg(not(feature = "nightly"))]
use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use syn::parse_macro_input;

#[cfg_attr(feature = "nightly", proc_macro)]
#[cfg_attr(not(feature = "nightly"), proc_macro_hack)]
pub fn pipe(input: TokenStream) -> TokenStream {
    let pipe = parse_macro_input!(input as Pipeline);
    pipe.into_token_stream().into()
}

mod pipeline;
