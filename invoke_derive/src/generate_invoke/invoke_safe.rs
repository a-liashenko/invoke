use std::str::FromStr;

use crate::{function_def::FunctionDef, invoke_ctx::InvokeCtx};
use syn::TypeTuple;

fn gen_any_cast(func: &FunctionDef) -> quote::__private::TokenStream {
    match &func.args {
        None => quote::quote! {},
        Some(v) => {
            let check = quote::quote! {
                let args = match args {
                    Some(v) => v,
                    None => return Err(::invoke::InvokeError::NoneArgs),
                };
            };

            if v.len() > 1 {
                let tuple = TypeTuple {
                    elems: v.clone(),
                    paren_token: Default::default(),
                };

                quote::quote! {
                    #check
                    let args: &#tuple = match args.downcast_ref() {
                        Some(v) => v,
                        None => return Err(::invoke::InvokeError::BadArgs),
                    };
                }
            } else {
                let arg = v.first();
                quote::quote! {
                    #check
                    let args: &#arg = match args.downcast_ref() {
                        Some(v) => v,
                        None => return Err(::invoke::InvokeError::BadArgs),
                    };
                }
            }
        }
    }
}

fn gen_fn_call(func: &FunctionDef) -> quote::__private::TokenStream {
    if func.args.is_some() {
        if func.args_amount == 1 {
            quote::quote! { args }
        } else {
            let range = (0..func.args_amount)
                .map(|v| format!("&args.{}", v))
                .collect::<Vec<_>>()
                .join(",");

            quote::__private::TokenStream::from_str(&range).unwrap()
        }
    } else {
        quote::quote! {}
    }
}

fn invoke_impl(fns: &[FunctionDef]) -> quote::__private::TokenStream {
    let mut ids = Vec::with_capacity(fns.len());
    let mut names = Vec::with_capacity(fns.len());
    let mut safe_cast = Vec::with_capacity(fns.len());
    let mut fn_args = Vec::with_capacity(fns.len());

    for f in fns {
        ids.push(f.id);
        names.push(&f.name_ident);
        safe_cast.push(gen_any_cast(f));
        fn_args.push(gen_fn_call(f));
    }

    let stream = quote::quote! {
        match fn_id {
            #(
                #ids => {
                    #safe_cast
                    self.#names(#fn_args);
                }
            )*
            _ => return Err(::invoke::InvokeError::UnknownMethod),
        };

    };

    stream
}

pub fn invoke(ctx: &InvokeCtx) -> quote::__private::TokenStream {
    let invoke_impl = invoke_impl(&ctx.immutable);
    let stream = quote::quote! {
        fn invoke(&self, fn_id: invoke::FnId, args: Option<&dyn std::any::Any>) -> Result<(), invoke::InvokeError> {
            #invoke_impl

            #[allow(unreachable_code)]
            Ok(())
        }
    };

    stream
}

pub fn invoke_mut(ctx: &InvokeCtx) -> quote::__private::TokenStream {
    let invoke_impl = invoke_impl(&ctx.mutable);
    let stream = quote::quote! {
        fn invoke_mut(&mut self, fn_id: invoke::FnId, args: Option<&dyn std::any::Any>) -> Result<(), invoke::InvokeError> {
            #invoke_impl

            #[allow(unreachable_code)]
            Ok(())
        }
    };

    stream
}
