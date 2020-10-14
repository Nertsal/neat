use super::*;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Gene {
    pub innovation_number: usize,
}

impl Gene {
    pub fn new(innovation_number: usize) -> Self {
        Self { innovation_number }
    }
}
