use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

pub mod calculations;
pub mod structure;

use calculations::*;
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
    pub probability_mutate_link: f32,
    pub probability_mutate_node: f32,
    pub probability_mutate_weight_shift: f32,
    pub probability_mutate_weight_random: f32,
    pub probability_mutate_link_toggle: f32,
    pub weight_shift_strength: f32,
    pub clients_mutation_rate: f32,
}

pub struct Neat {
    pub config: NeatConfig,
    pub clients: Vec<Rc<RefCell<Client>>>,
}

impl Neat {
    pub fn new(neat_config: NeatConfig) -> Rc<RefCell<Self>> {
        let clients_count = neat_config.max_clients;
        let neat = Rc::new(RefCell::new(Self {
            config: neat_config,
            clients: Vec::with_capacity(clients_count),
        }));
        for _ in 0..clients_count {
            let client = Client::new(Genome::empty(&neat));
            neat.borrow_mut()
                .clients
                .push(Rc::new(RefCell::new(client)));
        }
        neat
    }
    pub fn evolve(&mut self) {
        self.gen_species();
        self.kill();
        self.remove_extinct_species();
        self.reproduce();
        self.mutate();
    }
    fn gen_species(&mut self) {}
    fn kill(&mut self) {}
    fn remove_extinct_species(&mut self) {}
    fn reproduce(&mut self) {}
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
            client.borrow_mut().genome.mutate(&self.config);
        }
    }
}
