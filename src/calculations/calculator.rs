use super::*;
use std::collections::{HashMap, HashSet};

pub struct Calculator {
    input_nodes: HashSet<NodeGene>,
    hidden_nodes: HashSet<NodeGene>,
    output_nodes: HashSet<NodeGene>,
    connections: HashSet<ConnectionGene>,
}

impl Calculator {
    pub fn new(genome: &Genome) -> Self {
        let mut input_nodes = HashSet::new();
        let mut hidden_nodes = HashSet::new();
        let mut output_nodes = HashSet::new();
        for node in &genome.nodes {
            if node.x <= 0.0 {
                input_nodes.insert(node.clone());
            } else if node.x >= 1.0 {
                output_nodes.insert(node.clone());
            } else {
                hidden_nodes.insert(node.clone());
            }
        }

        let mut connections = HashSet::new();
        for connection in &genome.connections {
            connections.insert(connection.clone());
        }

        Self {
            input_nodes,
            hidden_nodes,
            output_nodes,
            connections,
        }
    }
    pub fn calculate(&self, input: Vec<f32>) -> Vec<f32> {
        let mut nodes_output = HashMap::new();

        for (index, input_node) in self.input_nodes.iter().enumerate() {
            nodes_output.insert(input_node.gene, input[index]);
        }

        for hidden_node in &self.hidden_nodes {
            nodes_output.insert(
                hidden_node.gene,
                hidden_node.calculate(&self.connections, &nodes_output),
            );
        }

        let mut output = Vec::with_capacity(self.output_nodes.len());
        for output_node in &self.output_nodes {
            let value = output_node.calculate(&self.connections, &nodes_output);
            nodes_output.insert(output_node.gene, value);
            output.push(value);
        }

        output
    }
}
