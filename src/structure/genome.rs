use super::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};

pub struct Genome {
    neat: Weak<RefCell<Neat>>,
    nodes: HashSet<NodeGene>,
    connections: Vec<ConnectionGene>,
}

impl Genome {
    pub fn empty(neat: &Rc<RefCell<Neat>>) -> Self {
        let mut nodes = HashSet::new();
        let config = &neat.borrow().config;
        for i in 0..(config.input_size + config.output_size) {
            nodes.insert(NodeGene::new(i));
        }
        Self {
            neat: Rc::downgrade(neat),
            nodes,
            connections: Vec::new(),
        }
    }
    pub fn distance(&self, other: &Self) -> f32 {
        let highest_gene1 = if let Some(gene) = self.connections.last() {
            gene.innovation_number
        } else {
            0
        };
        let highest_gene2 = if let Some(gene) = other.connections.last() {
            gene.innovation_number
        } else {
            0
        };

        let (genome1, genome2) = if highest_gene1 >= highest_gene2 {
            (self, other)
        } else {
            (other, self)
        };

        let mut index1 = 0;
        let mut index2 = 0;
        let mut disjoint = 0;
        let mut weight_diff = 0.0;
        let mut similar = 0;

        while index1 < genome1.connections.len() && index2 < genome2.connections.len() {
            let con1 = genome1.connections.get(index1).unwrap();
            let con2 = genome2.connections.get(index2).unwrap();

            if con1.innovation_number == con2.innovation_number {
                similar += 1;
                weight_diff += (con1.weight - con2.weight).abs();
                index1 += 1;
                index2 += 1;
            } else if con1.innovation_number > con2.innovation_number {
                disjoint += 1;
                index2 += 1;
            } else {
                disjoint += 1;
                index1 += 1;
            }
        }

        weight_diff /= similar.max(1) as f32;
        let excess = genome1.connections.len() - index1;

        let n = genome1.connections.len().max(genome2.connections.len()) as f32;
        let neat = self.neat.upgrade().unwrap();
        let neat_config = &neat.borrow().config;

        (neat_config.disjoint * disjoint as f32
            + neat_config.excess * excess as f32
            + neat_config.weight_diff * weight_diff)
            / n
    }
}
