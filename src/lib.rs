use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub mod structure;

use structure::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NeatConfig {
    pub input_size: usize,
    pub output_size: usize,
    pub max_clients: usize,
    pub disjoint: f32,
    pub excess: f32,
    pub weight_diff: f32,
    pub cp: f32,
    pub cp_change_rate: f32,
    pub target_species_count: usize,
    pub probability_mutate_link: f32,
    pub probability_mutate_node: f32,
    pub probability_mutate_weight_shift: f32,
    pub probability_mutate_weight_random: f32,
    pub probability_mutate_link_toggle: f32,
    pub weight_shift_strength: f32,
    pub weight_random_strength: f32,
    pub clients_mutation_rate: f32,
    pub survivors_percentage: f32,
}

#[derive(Debug)]
pub struct Neat {
    pub config: NeatConfig,
    pub clients: Vec<Rc<RefCell<Client>>>,
    pub connection_genes: HashSet<ConnectionGene>,
    pub species: Vec<Species>,
}

impl Neat {
    pub fn new(neat_config: NeatConfig) -> Rc<RefCell<Self>> {
        for _ in 0..(neat_config.input_size + neat_config.output_size) {
            Gene::new();
        }

        let clients_count = neat_config.max_clients;
        let mut neat = Self {
            config: neat_config,
            clients: Vec::with_capacity(clients_count),
            connection_genes: HashSet::new(),
            species: Vec::new(),
        };
        for _ in 0..clients_count {
            let client = Client::new(Genome::start(&mut neat));
            neat.clients.push(Rc::new(RefCell::new(client)));
        }
        Rc::new(RefCell::new(neat))
    }
    pub fn evolve(&mut self) {
        println!("Generating species...");
        self.gen_species();
        println!("{} species exist.", self.species.len());
        println!("Killing worst clients...");
        self.kill();
        println!("Remove extinct species...");
        self.remove_extinct_species();
        println!("{} species left.", self.species.len());
        println!("Reproducing...");
        self.reproduce();
        println!("Mutating clients...");
        self.mutate();
    }
    fn gen_species(&mut self) {
        for species in &mut self.species {
            species.reset();
        }
        for client in &self.clients {
            if self
                .species
                .iter()
                .any(|species| species.clients.contains(client))
            {
                continue;
            }
            let mut found = false;
            for species in &mut self.species {
                if species.insert(client, &self.config) {
                    found = true;
                    break;
                }
            }
            if !found {
                self.species.push(Species {
                    clients: vec![client.clone()],
                    representative: client.clone(),
                    score: 0.0,
                });
            }
        }
        for species in &mut self.species {
            species.evaluate_score();
        }
        self.config.cp += self.config.cp_change_rate
            * (self.species.len() as f32 - self.config.target_species_count as f32);
        self.config.cp = self.config.cp.max(0.1);
    }
    fn kill(&mut self) {
        for species in &mut self.species {
            species.kill(1.0 - self.config.survivors_percentage);
        }
    }
    fn remove_extinct_species(&mut self) {
        for i in (0..self.species.len()).rev() {
            if self.species[i].clients.len() <= 1 {
                self.species.remove(i);
            }
        }
    }
    fn reproduce(&mut self) {
        for client in &mut self.clients {
            if self
                .species
                .iter()
                .any(|species| species.clients.contains(client))
            {
                continue;
            }
            let mut random = rand::thread_rng();
            match self
                .species
                .choose_weighted_mut(&mut random, |species| species.score)
            {
                Ok(species) => {
                    client.borrow_mut().genome = species.breed(&self.config);
                    species.insert_force(client);
                }
                Err(_) => (),
            }
        }
    }
    fn mutate(&mut self) {
        self.clients.sort_by(|client1, client2| {
            client2
                .borrow()
                .score
                .partial_cmp(&client1.borrow().score)
                .unwrap()
        });
        let skip = (self.clients.len() as f32 * (1.0 - self.config.clients_mutation_rate)) as usize;
        for client in self.clients.iter_mut().skip(skip) {
            client
                .borrow_mut()
                .genome
                .mutate(&self.config, &mut self.connection_genes);
        }
    }
    fn get_connection_gene(
        connection_genes: &mut HashSet<ConnectionGene>,
        node_from: NodeGene,
        node_to: NodeGene,
        weight: f32,
        enabled: bool,
    ) -> ConnectionGene {
        if let Some(connection) = connection_genes
            .iter()
            .find(|connection| connection.node_from == node_from && connection.node_to == node_to)
        {
            connection.clone()
        } else {
            let connection = ConnectionGene::new(Gene::new(), node_from, node_to, weight, enabled);
            connection_genes.insert(connection);
            connection
        }
    }
}
