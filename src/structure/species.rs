use super::*;
use rand::Rng;

#[derive(Debug)]
pub struct Species {
    pub clients: Vec<Rc<Client>>,
    pub representative: Rc<Client>,
    pub score: f32,
}

impl Species {
    pub fn new(representative: &mut Rc<Client>) -> Self {
        let mut clients = Vec::new();
        clients.push(representative.clone());
        Self {
            clients,
            representative: representative.clone(),
            score: 0.0,
        }
    }
    pub fn insert(&mut self, client: &mut Rc<Client>) -> bool {
        if client.genome.distance(&self.representative.genome)
            < self
                .representative
                .genome
                .neat
                .upgrade()
                .unwrap()
                .borrow()
                .config
                .cp
        {
            self.clients.push(client.clone());
            true
        } else {
            false
        }
    }
    pub fn insert_force(&mut self, client: &mut Rc<Client>) {
        self.clients.push(client.clone());
    }
    pub fn get_score(&self) -> f32 {
        self.clients.iter().map(|client| client.score).sum()
    }
    pub fn kill(&mut self, percentage: f32) {
        self.clients
            .sort_by(|client1, client2| client1.score.partial_cmp(&client2.score).unwrap());
        for _ in 0..((self.clients.len() as f32 * percentage) as usize) {
            self.clients.remove(0);
        }
    }
    pub fn breed(&self) -> Genome {
        let client1 = &self.clients[rand::thread_rng().gen_range(0, self.clients.len())];
        let client2 = &self.clients[rand::thread_rng().gen_range(0, self.clients.len())];
        if client1.score > client2.score {
            Genome::cross_over(&client1.genome, &client2.genome)
        } else {
            Genome::cross_over(&client2.genome, &client1.genome)
        }
    }
}
