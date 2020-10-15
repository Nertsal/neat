#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Gene {
    pub innovation_number: usize,
}

impl Gene {
    pub fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
        Self {
            innovation_number: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}
