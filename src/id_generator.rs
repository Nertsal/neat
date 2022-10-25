use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdGenerator {
    next_id: Id,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u64);

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: Id(0) }
    }

    pub fn gen(&mut self) -> Id {
        let id = self.next_id;
        self.next_id.0 += 1;
        id
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}
