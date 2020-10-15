use std::cell::RefCell;
use std::rc::Rc;

pub mod calculations;
pub mod structure;

use calculations::*;
use structure::*;

pub struct NeatConfig {
    pub input_size: usize,
    pub output_size: usize,
    pub max_clients: usize,
    pub disjoint: f32,
    pub excess: f32,
    pub weight_diff: f32,
}

pub struct Neat {
    pub config: NeatConfig,
    pub clients: Vec<Rc<Client>>,
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
            neat.borrow_mut().clients.push(Rc::new(client));
        }
        neat
    }
}
