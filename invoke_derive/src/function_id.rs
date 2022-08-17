use convert_case::{Case, Casing};
use quote::{format_ident, ToTokens};
use syn::{token, Ident};

#[derive(Debug, Clone)]
pub struct FunctionId {
    uuid: uuid::Uuid,
    pub name: Ident,
}

impl FunctionId {
    pub fn new(name: &Ident) -> Self {
        let name = format_ident!("{}", format!("{}_ID", name).to_case(Case::ScreamingSnake));
        let uuid = uuid::Uuid::new_v4();
        Self { uuid, name }
    }

    pub fn get_id(&self) -> Id {
        Id(self.uuid)
    }
}

impl ToTokens for FunctionId {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let bytes = self.uuid.as_bytes();

        token::Bracket::default().surround(tokens, |tokens| {
            for byte in bytes {
                byte.to_tokens(tokens);
                token::Comma::default().to_tokens(tokens);
            }
        })
    }
}

pub struct Id(uuid::Uuid);
impl ToTokens for Id {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let bytes = self.0.as_bytes();

        token::Bracket::default().surround(tokens, |tokens| {
            for byte in bytes {
                byte.to_tokens(tokens);
                token::Comma::default().to_tokens(tokens);
            }
        })
    }
}
