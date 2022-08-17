pub use memx;

mod invoke_meta;
mod invoke_trait;

pub use invoke_derive::invoke;
pub use invoke_meta::{InvokeMeta, InvokeMetaExt};
pub use invoke_trait::{Invoke, InvokeExt, InvokeMut, InvokeMutExt};
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct FnId(pub [u8; 16]);

impl FnId {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for FnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid = Uuid::from_bytes(self.0);
        f.debug_struct("FnId").field("uuid", &uuid).finish()
    }
}

#[derive(Debug)]
pub enum InvokeError {
    NoneArgs,
    BadArgs,
    UnknownMethod,
}
