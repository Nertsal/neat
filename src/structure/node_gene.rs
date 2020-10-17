use super::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct NodeGene {
    pub gene: Gene,
    pub x: f32,
    pub y: f32,
}

impl NodeGene {
    pub fn new(innovation_number: usize, x: f32, y: f32) -> Self {
        Self {
            gene: Gene { innovation_number },
            x,
            y,
        }
    }
    pub fn calculate(
        &self,
        connections: &Vec<ConnectionGene>,
        output: &mut HashMap<Gene, f32>,
    ) -> f32 {
        Self::activate(
            connections
                .iter()
                .filter(|connection| connection.enabled && connection.node_to == *self)
                .map(|connection| {
                    let value = match output.get(&connection.node_from.gene) {
                        Some(value) => connection.weight * value,
                        None => connection.node_from.calculate(connections, output),
                    };
                    output.insert(self.gene, value);
                    value
                })
                .sum(),
        )
    }
    fn activate(value: f32) -> f32 {
        1.0 / (1.0 + (-value).exp())
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
