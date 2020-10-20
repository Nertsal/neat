use super::*;
use rand::Rng;

#[derive(Debug)]
pub struct Species {
    pub clients: Vec<Rc<RefCell<Client>>>,
    pub representative: Rc<RefCell<Client>>,
    pub score: f32,
}

impl Species {
    pub fn new(representative: &mut Rc<RefCell<Client>>) -> Self {
        let mut clients = Vec::new();
        clients.push(representative.clone());
        Self {
            clients,
            representative: representative.clone(),
            score: 0.0,
        }
    }
    pub fn insert(&mut self, client: &Rc<RefCell<Client>>, neat_config: &NeatConfig) -> bool {
        if client
            .borrow()
            .genome
            .distance(&self.representative.borrow().genome, neat_config)
            < neat_config.cp
        {
            self.clients.push(client.clone());
            true
        } else {
            false
        }
    }
    pub fn insert_force(&mut self, client: &mut Rc<RefCell<Client>>) {
        self.clients.push(client.clone());
    }
    pub fn reset(&mut self) {
        let mut random = rand::thread_rng();
        let representative = self.clients.remove(random.gen_range(0, self.clients.len()));
        self.clients.clear();
        self.clients.push(representative);
        self.score = 0.0;
    }
    pub fn evaluate_score(&mut self) {
        self.score = self
            .clients
            .iter()
            .map(|client| client.borrow().score)
            .sum();
    }
    pub fn kill(&mut self, percentage: f32) {
        self.clients.sort_by(|client1, client2| {
            client1
                .borrow()
                .score
                .partial_cmp(&client2.borrow().score)
                .unwrap()
        });
        for _ in 0..((self.clients.len() as f32 * percentage).floor() as usize) {
            self.clients.remove(0);
        }
    }
    pub fn breed(&self, neat_config: &NeatConfig) -> Genome {
        let client1 = &self.clients[rand::thread_rng().gen_range(0, self.clients.len())];
        let client2 = &self.clients[rand::thread_rng().gen_range(0, self.clients.len())];
        if client1.borrow().score > client2.borrow().score {
            Genome::cross_over(
                &client1.borrow().genome,
                &client2.borrow().genome,
                neat_config,
            )
        } else {
            Genome::cross_over(
                &client2.borrow().genome,
                &client1.borrow().genome,
                neat_config,
            )
        }
    }
}
