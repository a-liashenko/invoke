use std::str::FromStr;

use crate::{function_def::FunctionDef, invoke_ctx::InvokeCtx};
use syn::TypeTuple;

fn gen_unsafe_cast(func: &FunctionDef) -> quote::__private::TokenStream {
    match &func.args {
        None => quote::quote! {},
        Some(v) => {
            if v.len() > 1 {
                let tuple = TypeTuple {
                    elems: v.clone(),
                    paren_token: Default::default(),
                };

                quote::quote! {
                    let args: *const #tuple = std::mem::transmute(args);
                }
            } else {
                let arg = v.first();
                quote::quote! {
                    let args: *const #arg = std::mem::transmute(args);
                }
            }
        }
    }
}

fn gen_fn_call(func: &FunctionDef) -> quote::__private::TokenStream {
    if func.args.is_some() {
        if func.args_amount == 1 {
            quote::quote! { &*args }
        } else {
            let range = (0..func.args_amount)
                .map(|v| format!("&(*args).{}", v))
                .collect::<Vec<_>>()
                .join(",");

            quote::__private::TokenStream::from_str(&range).unwrap()
        }
    } else {
        quote::quote! {}
    }
}

fn invoke_raw_impl(fns: &[FunctionDef]) -> quote::__private::TokenStream {
    let mut ids = Vec::with_capacity(fns.len());
    let mut names = Vec::with_capacity(fns.len());
    let mut unsafe_cast = Vec::with_capacity(fns.len());
    let mut fn_args = Vec::with_capacity(fns.len());

    for f in fns {
        ids.push(f.id);
        names.push(&f.name_ident);
        unsafe_cast.push(gen_unsafe_cast(f));
        fn_args.push(gen_fn_call(f));
    }

    let stream = quote::quote! {
        #[allow(unreachable_code)]
        match fn_id {
            #(
                #ids => {
                    #unsafe_cast
                    self.#names(#fn_args);
                }
            )*
            _ => return Err(::invoke::InvokeError::UnknownMethod),
        };
    };

    stream
}

pub fn invoke_raw(ctx: &InvokeCtx) -> quote::__private::TokenStream {
    let invoke_impl = invoke_raw_impl(&ctx.immutable);
    let stream = quote::quote! {
        unsafe fn invoke_raw_ptr(&self, fn_id: u16, args: *const ::std::ffi::c_void) -> Result<(), ::invoke::InvokeError> {
            #invoke_impl

            #[allow(unreachable_code)]
            Ok(())
        }
    };

    stream
}

pub fn invoke_raw_mut(ctx: &InvokeCtx) -> quote::__private::TokenStream {
    let invoke_impl = invoke_raw_impl(&ctx.mutable);
    let stream = quote::quote! {
        unsafe fn invoke_raw_ptr_mut(&mut self, fn_id: u16, args: *const ::std::ffi::c_void) -> Result<(), ::invoke::InvokeError> {
            #invoke_impl

            #[allow(unreachable_code)]
            Ok(())
        }
    };

    stream
}
