use super::*;

#[derive(Copy, Clone, Debug)]
pub struct NodeGene {
    pub gene: Gene,
    pub x: f32,
    pub y: f32,
}

impl NodeGene {
    pub fn new(innovation_number: usize) -> Self {
        Self {
            gene: Gene::new(innovation_number),
            x: 0.0,
            y: 0.0,
        }
    }
}

impl PartialEq for NodeGene {
    fn eq(&self, other: &Self) -> bool {
        self.gene == other.gene
    }
}

impl Eq for NodeGene {}

impl std::hash::Hash for NodeGene {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.gene.hash(state);
    }
}
