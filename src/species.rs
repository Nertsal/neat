use super::*;
use rand::Rng;

#[derive(Debug)]
pub struct Species {
    pub clients: Vec<Id>,
    pub representative: Id,
    pub score: f32,
}

impl Species {
    pub fn new(representative: Id) -> Self {
        Self {
            clients: vec![representative],
            representative,
            score: 0.0,
        }
    }

    pub fn insert(
        &mut self,
        client: &Client,
        all_clients: &HashMap<Id, Client>,
        neat_config: &NeatConfig,
    ) -> bool {
        if client.genome.distance(
            &all_clients.get(&self.representative).unwrap().genome,
            neat_config,
        ) < neat_config.cp
        {
            self.clients.push(client.id);
            true
        } else {
            false
        }
    }

    pub fn insert_force(&mut self, client: &Client) {
        self.clients.push(client.id);
    }

    pub fn reset(&mut self) {
        let mut random = rand::thread_rng();
        let representative = self.clients.remove(random.gen_range(0, self.clients.len()));
        self.clients.clear();
        self.clients.push(representative);
        self.score = 0.0;
    }

    pub fn evaluate_score(&mut self, all_clients: &HashMap<Id, Client>) {
        self.score = self
            .clients
            .iter()
            .map(|client| all_clients.get(client).unwrap().score)
            .sum();
    }

    pub fn kill(&mut self, all_clients: &HashMap<Id, Client>, percentage: f32) {
        self.clients.sort_by(|client1, client2| {
            all_clients
                .get(client1)
                .unwrap()
                .score
                .partial_cmp(&all_clients.get(client2).unwrap().score)
                .unwrap()
        });
        for _ in 0..((self.clients.len() as f32 * percentage).floor() as usize) {
            self.clients.remove(0);
        }
    }

    pub fn breed(&self, all_clients: &HashMap<Id, Client>, neat_config: &NeatConfig) -> Genome {
        let client1 = &self
            .clients
            .get(rand::thread_rng().gen_range(0, self.clients.len()))
            .unwrap();
        let client1 = all_clients.get(client1).unwrap();
        let client2 = &self
            .clients
            .get(rand::thread_rng().gen_range(0, self.clients.len()))
            .unwrap();
        let client2 = all_clients.get(client2).unwrap();
        if client1.score > client2.score {
            Genome::cross_over(&client1.genome, &client2.genome, neat_config)
        } else {
            Genome::cross_over(&client2.genome, &client1.genome, neat_config)
        }
    }
}
