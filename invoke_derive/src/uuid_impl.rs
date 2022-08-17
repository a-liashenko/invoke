use quote::ToTokens;
use syn::token;

#[derive(Debug, Clone, Copy)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl ToTokens for Uuid {
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
