use quote::format_ident;
use syn::{punctuated::Punctuated, Ident, Token, Type};

pub type ArgTypes = Punctuated<Type, Token![,]>;

pub struct FunctionDef {
    // TODO: Replace with random numbers?
    pub id: u16,
    pub name: String,
    pub id_ident: Ident,
    pub name_ident: Ident,
    pub args: Option<ArgTypes>,
    pub args_amount: u8,
}

impl std::fmt::Debug for FunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDef")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("id_ident", &self.id_ident)
            .field("name_ident", &self.name_ident)
            .field("args_amount", &self.args_amount)
            .finish()
    }
}

impl FunctionDef {
    pub fn new(id: u16, name: &Ident, args: Option<ArgTypes>, amount: u8) -> Self {
        use convert_case::*;
        let ident = format_ident!("{}", format!("{}_ID", name).to_case(Case::ScreamingSnake));

        Self {
            id,
            name_ident: name.clone(),
            id_ident: ident,
            name: name.to_string(),
            args,
            args_amount: amount,
        }
    }

    pub fn to_enum_entry(&self) -> quote::__private::TokenStream {
        let id = self.id;
        let ident = &self.id_ident;

        quote::quote! { const #ident: u16 = #id; }
    }
}
