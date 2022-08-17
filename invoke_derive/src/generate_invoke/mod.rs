mod invoke_raw;
mod invoke_safe;

use crate::invoke_ctx::InvokeCtx;
use proc_macro::TokenStream;
use quote::format_ident;

pub fn generate_invoke(ctx: &InvokeCtx) -> TokenStream {
    let struct_ident = format_ident!("{}", ctx.name);
    let invoke_raw = invoke_raw::invoke_raw(ctx);
    let invoke_raw_mut = invoke_raw::invoke_raw_mut(ctx);

    let invoke = invoke_safe::invoke(ctx);
    let invoke_mut = invoke_safe::invoke_mut(ctx);

    let stream = quote::quote! {
        impl invoke::Invoke for #struct_ident {
            #invoke
            #invoke_raw
        }

        impl invoke::InvokeMut for #struct_ident {
            #invoke_mut
            #invoke_raw_mut
        }

    };

    stream.into()
}
