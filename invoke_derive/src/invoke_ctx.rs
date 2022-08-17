use quote::{format_ident, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, FnArg, Ident, ImplItem, ImplItemMethod, ItemImpl, Signature, Token, Type};

use crate::function_def::FunctionDef;
use crate::function_id::FunctionId;

enum FnType {
    General,
    Mutable,
    Immutable,
}

#[derive(Debug)]
pub struct InvokeCtx {
    pub name: String,
    pub mod_ident: Ident,
    pub mutable: Vec<FunctionDef>,
    pub immutable: Vec<FunctionDef>,
}

impl InvokeCtx {
    pub fn from_impl(impl_block: &mut ItemImpl) -> syn::Result<InvokeCtx> {
        let name = impl_block.self_ty.to_token_stream().to_string();
        let mut ctx = Self {
            mod_ident: mod_ident(&name),
            name,
            immutable: Default::default(),
            mutable: Default::default(),
        };

        let methods = impl_block.items.iter_mut().filter_map(|v| match v {
            ImplItem::Method(v) => Some(v),
            _ => None,
        });

        for f in methods {
            let fn_type = validate_function(f)?;
            if let FnType::General = fn_type {
                continue;
            }

            let id = FunctionId::new(&f.sig.ident);
            let (args_amount, args) = parse_args(&f.sig.inputs)?;
            let def = FunctionDef::new(id, &f.sig.ident, args, args_amount);

            match fn_type {
                FnType::Mutable => ctx.mutable.push(def),
                FnType::Immutable => ctx.immutable.push(def),
                _ => unreachable!(),
            }
        }
        Ok(ctx)
    }
}

fn get_invoke_idx(attrs: &[Attribute]) -> Option<usize> {
    attrs.iter().position(|v| {
        if v.path.leading_colon.is_some() {
            return false;
        }

        if v.path.segments.len() > 1 {
            return false;
        }

        if let Some(name) = v.path.segments.first() {
            name.ident == "invoke_fn"
        } else {
            false
        }
    })
}

fn mod_ident(name: &str) -> Ident {
    use convert_case::*;
    format_ident!("{}", format!("{}_meta", name).to_case(Case::Snake))
}

fn validate_arg(arg: &FnArg) -> syn::Result<FnType> {
    if let FnArg::Receiver(v) = arg {
        if v.reference.is_some() {
            return match v.mutability.is_some() {
                true => Ok(FnType::Mutable),
                false => Ok(FnType::Immutable),
            };
        }
    }

    Err(syn::Error::new(
        arg.span(),
        "Supported only &self or &mut self functions",
    ))
}

fn parse_args(
    args: &Punctuated<FnArg, Token![,]>,
) -> syn::Result<(u8, Option<Punctuated<Type, Token![,]>>)> {
    let mut args_types: Punctuated<Type, Token![,]> = Default::default();
    for arg in args.iter() {
        if let FnArg::Typed(arg) = arg {
            match arg.ty.as_ref() {
                Type::Reference(v) => {
                    if v.mutability.is_some() {
                        return Err(syn::Error::new(
                            v.mutability.span(),
                            "Only immutable refs allowed",
                        ));
                    }
                    args_types.push(*v.elem.clone());
                }
                _ => return Err(syn::Error::new(arg.ty.span(), "Only refs allowed")),
            }
        }
    }

    let res = match args_types.len() {
        0 => (0, None),
        len => (len as u8, Some(args_types)),
    };

    Ok(res)
}

fn validate_signature(sig: &Signature) -> syn::Result<FnType> {
    if sig.asyncness.is_some() {
        return Err(syn::Error::new(
            sig.asyncness.span(),
            "Async function not supported",
        ));
    }

    match sig.inputs.first() {
        Some(arg) => validate_arg(arg),
        None => Err(syn::Error::new(
            sig.paren_token.span,
            "Expected &self or &mut self",
        )),
    }
}

fn validate_function(func: &mut ImplItemMethod) -> syn::Result<FnType> {
    let invoke_idx = match get_invoke_idx(&func.attrs) {
        Some(idx) => idx,
        None => return Ok(FnType::General),
    };

    let fn_type = validate_signature(&func.sig)?;
    if let FnType::General = &fn_type {
        panic!("Unexpected FnType::General received");
    }

    func.attrs.remove(invoke_idx);
    Ok(fn_type)
}
