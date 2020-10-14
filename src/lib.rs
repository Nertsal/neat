use std::cell::RefCell;
use std::rc::Rc;

mod calculations;
mod structure;

pub use calculations::*;
pub use structure::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let neat_config = NeatConfig {
            input_size: 5,
            output_size: 1,
            max_clients: 5,
            disjoint: 1.0,
            excess: 1.0,
            weight_diff: 1.0,
        };
        let neat = Neat::new(neat_config);
    }
}
