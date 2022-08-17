use std::hash::Hash;

#[derive(Clone, Copy, Eq)]
pub struct FnId(pub [u8; 16]);

impl FnId {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl PartialEq for FnId {
    fn eq(&self, other: &Self) -> bool {
        memx::memeq(&self.0, &other.0)
    }
}

impl PartialOrd for FnId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FnId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        memx::memcmp(&self.0, &other.0)
    }
}

impl Hash for FnId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

impl std::fmt::Debug for FnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid = uuid::Uuid::from_bytes(self.0);
        f.debug_struct("FnId").field("uuid", &uuid).finish()
    }
}
