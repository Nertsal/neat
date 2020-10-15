use super::*;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
pub struct NodeGene {
    pub gene: Gene,
    pub x: f32,
    pub y: f32,
}

impl NodeGene {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            gene: Gene::new(),
            x,
            y,
        }
    }
    pub fn calculate(
        &self,
        connections: &HashSet<ConnectionGene>,
        output: &HashMap<Gene, f32>,
    ) -> f32 {
        connections
            .iter()
            .filter(|connection| connection.enabled && connection.node_to == *self)
            .map(|connection| connection.weight * output[&connection.node_from.gene])
            .sum()
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
