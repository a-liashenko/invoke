use crate::function_id::FunctionId;
use syn::{punctuated::Punctuated, Ident, Token, Type};

pub type ArgTypes = Punctuated<Type, Token![,]>;

pub struct FunctionDef {
    pub id: FunctionId,
    pub name: String,
    pub name_ident: Ident,
    pub args: Option<ArgTypes>,
    pub args_amount: u8,
}

impl std::fmt::Debug for FunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDef")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("name_ident", &self.name_ident)
            .field("args_amount", &self.args_amount)
            .finish()
    }
}

impl FunctionDef {
    pub fn new(id: FunctionId, name: &Ident, args: Option<ArgTypes>, amount: u8) -> Self {
        Self {
            id,
            name_ident: name.clone(),
            name: name.to_string(),
            args,
            args_amount: amount,
        }
    }

    pub fn to_enum_entry(&self) -> quote::__private::TokenStream {
        let id = self.id.get_id();
        let ident = &self.id.name;

        quote::quote! { const #ident: invoke::FnId = invoke::FnId(#id); }
    }
}
