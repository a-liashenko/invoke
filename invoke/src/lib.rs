mod invoke_meta;
mod invoke_trait;

pub use invoke_derive::invoke;
pub use invoke_meta::{InvokeMeta, InvokeMetaExt};
pub use invoke_trait::{Invoke, InvokeExt};

pub type FnId = u16;

#[derive(Debug)]
pub enum InvokeError {
    NoneArgs,
    BadArgs,
    UnknownMethod,
}
