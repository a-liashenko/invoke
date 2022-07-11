use proc_macro::TokenStream;

use crate::{function_def::FunctionDef, invoke_ctx::InvokeCtx};

fn generate_fn_ids(fns: &[FunctionDef]) -> quote::__private::TokenStream {
    let values = fns.iter().map(|v| v.to_enum_entry()).collect::<Vec<_>>();
    quote::quote! { #(pub #values)* }
}

pub fn generate_ids(ctx: &InvokeCtx) -> TokenStream {
    let mod_name = &ctx.mod_ident;
    let stream = generate_fn_ids(&ctx.immutable);
    let stream_mut = generate_fn_ids(&ctx.mutable);

    let stream = quote::quote! {
        pub mod #mod_name {
            #stream
            #stream_mut
        }
    };

    stream.into()
}
