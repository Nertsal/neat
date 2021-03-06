use super::*;

#[derive(Copy, Clone, Debug)]
pub struct ConnectionGene {
    pub gene: Gene,
    pub node_from: NodeGene,
    pub node_to: NodeGene,
    pub weight: f32,
    pub enabled: bool,
    pub replace_gene: Option<Gene>,
}

impl ConnectionGene {
    pub fn new(
        gene: Gene,
        node_from: NodeGene,
        node_to: NodeGene,
        weight: f32,
        enabled: bool,
    ) -> Self {
        Self {
            gene,
            node_from,
            node_to,
            weight,
            enabled,
            replace_gene: None,
        }
    }
}

impl std::ops::Deref for ConnectionGene {
    type Target = Gene;

    fn deref(&self) -> &Self::Target {
        &self.gene
    }
}

impl PartialEq for ConnectionGene {
    fn eq(&self, other: &Self) -> bool {
        self.gene == other.gene
    }
}

impl Eq for ConnectionGene {}

impl std::hash::Hash for ConnectionGene {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.gene.hash(state);
    }
}
