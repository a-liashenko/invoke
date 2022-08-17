use proc_macro::TokenStream;
use quote::format_ident;

use crate::invoke_ctx::InvokeCtx;

pub fn generate_meta(ctx: &InvokeCtx) -> TokenStream {
    let name_ident = format_ident!("{}", ctx.name);

    let capacity = ctx.mutable.len() + ctx.immutable.len();
    let mut fns = Vec::with_capacity(capacity);
    let mut fns_names = Vec::with_capacity(capacity);
    let mut idx = Vec::with_capacity(capacity);

    for func in ctx.immutable.iter().chain(ctx.mutable.iter()) {
        fns.push(&func.name_ident);
        fns_names.push(format!("{}::{}", ctx.name, func.name));
        idx.push(&func.id.name);
    }

    quote::quote! {
        impl invoke::InvokeMeta for #name_ident {
            fn get_method_id_raw_ptr(func_ptr: *const std::ffi::c_void) -> Option<&'static invoke::FnId> {
                #(
                    if std::ptr::eq(func_ptr, &Self::#fns as *const _ as *const std::ffi::c_void) {
                        return Some(&Self::#idx);
                    }
                )*

                None
            }

            fn get_method_id(name: &str) -> Option<&'static invoke::FnId> {
                #[allow(unreachable_code)]
                match name {
                    #(#fns_names => Some(&Self::#idx),)*
                    _ => None
                }
            }

            fn get_method_name(id: &invoke::FnId) -> Option<&'static str> {
                #(
                    if id.eq(&Self::#idx) {
                        return Some(#fns_names);
                    }
                )*

                return None;
            }
        }
    }
    .into()
}
