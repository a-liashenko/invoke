mod function_def;
mod generate_ids;
mod generate_invoke;
mod generate_meta;
mod invoke_ctx;

use generate_ids::generate_ids;
use generate_invoke::generate_invoke;
use generate_meta::generate_meta;
use invoke_ctx::InvokeCtx;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemImpl};

#[proc_macro_attribute]
pub fn invoke(_args: TokenStream, token: TokenStream) -> TokenStream {
    let mut block: ItemImpl = parse_macro_input!(token);
    let ctx = match InvokeCtx::from_impl(&mut block) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    let mut base: TokenStream = block.into_token_stream().into();

    let enums = generate_ids(&ctx);
    let invoke = generate_invoke(&ctx);
    let meta = generate_meta(&ctx);

    base.extend(enums);
    base.extend(invoke);
    base.extend(meta);
    base
}
