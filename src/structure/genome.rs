use super::*;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};

pub struct Genome {
    pub neat: Weak<RefCell<Neat>>,
    pub nodes: HashSet<NodeGene>,
    pub connections: Vec<ConnectionGene>,
}

impl Genome {
    pub fn empty(neat: &Rc<RefCell<Neat>>) -> Self {
        let mut nodes = HashSet::new();
        let config = &neat.borrow().config;
        for i in 0..config.input_size {
            nodes.insert(NodeGene::new(
                0.0,
                (i as f32 + 1.0) / (config.input_size as f32 + 1.0),
            ));
        }
        for i in 0..config.output_size {
            nodes.insert(NodeGene::new(
                1.0,
                (i as f32 + 1.0) / (config.output_size as f32 + 1.0),
            ));
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
    pub fn cross_over(genome1: &Genome, genome2: &Genome) -> Genome {
        let mut genome = Genome::empty(&genome1.neat.upgrade().unwrap());

        let mut index1 = 0;
        let mut index2 = 0;

        while index1 < genome1.connections.len() && index2 < genome2.connections.len() {
            let con1 = genome1.connections.get(index1).unwrap();
            let con2 = genome2.connections.get(index2).unwrap();

            if con1.innovation_number == con2.innovation_number {
                if rand::thread_rng().gen_bool(0.5) {
                    genome.connections.push(con1.clone());
                } else {
                    genome.connections.push(con2.clone());
                }
                index1 += 1;
                index2 += 1;
            } else if con1.innovation_number > con2.innovation_number {
                index2 += 1;
            } else {
                genome.connections.push(con1.clone());
                index1 += 1;
            }
        }

        while index1 < genome1.connections.len() {
            let con1 = genome1.connections.get(index1).unwrap();
            genome.connections.push(con1.clone());
            index1 += 1;
        }

        for connection in &genome.connections {
            genome.nodes.insert(connection.node_from.clone());
            genome.nodes.insert(connection.node_to.clone());
        }

        genome
    }
    pub fn mutate(&mut self, neat: &NeatConfig) {
        let mut random = rand::thread_rng();
        if random.gen::<f32>() <= neat.probability_mutate_link {
            self.mutate_link(neat);
        }
        if random.gen::<f32>() <= neat.probability_mutate_node {
            self.mutate_node(neat);
        }
        if random.gen::<f32>() <= neat.probability_mutate_weight_shift {
            self.mutate_weight_shift(neat);
        }
        if random.gen::<f32>() <= neat.probability_mutate_weight_random {
            self.mutate_weight_random(neat);
        }
        if random.gen::<f32>() <= neat.probability_mutate_link_toggle {
            self.mutate_link_toggle();
        }
    }
    fn mutate_link(&mut self, neat: &NeatConfig) {
        let mut random = rand::thread_rng();
        if self.nodes.len() >= 2 {
            for _ in 0..10 {
                let index1 = random.gen_range(0, self.nodes.len());
                let index2 = random.gen_range(0, self.nodes.len());

                let mut node1 = None;
                let mut node2 = None;
                for (index, node) in self.nodes.iter().enumerate() {
                    if index == index1 {
                        node1 = Some(node.clone());
                    }
                    if index == index2 {
                        node2 = Some(node.clone());
                    }
                }

                let node1 = node1.unwrap();
                let node2 = node2.unwrap();

                if node1.x == node2.x {
                    continue;
                }

                let (node_from, node_to) = if node1.x < node2.x {
                    (node1, node2)
                } else {
                    (node2, node1)
                };

                if self.connections.iter().any(|connection| {
                    connection.node_from == node_from && connection.node_to == node_to
                }) {
                    continue;
                }

                let connection = ConnectionGene {
                    gene: Gene::new(),
                    node_from,
                    node_to,
                    weight: (random.gen::<f32>() * 2.0 - 1.0) * neat.weight_shift_strength,
                    enabled: true,
                };
                self.connections.push(connection);
            }
        }
    }
    fn mutate_node(&mut self, neat: &NeatConfig) {}
    fn mutate_weight_shift(&mut self, neat: &NeatConfig) {}
    fn mutate_weight_random(&mut self, neat: &NeatConfig) {}
    fn mutate_link_toggle(&mut self) {}
}
