use super::*;

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
    pub id_gen: IdGenerator,
    pub config: NeatConfig,
    pub clients: HashMap<Id, Client>,
    pub connection_genes: HashSet<ConnectionGene>,
    pub species: Vec<Species>,
}

impl Neat {
    pub fn new(neat_config: NeatConfig) -> Self {
        for _ in 0..(neat_config.input_size + neat_config.output_size) {
            Gene::new();
        }

        let clients_count = neat_config.max_clients;
        let mut neat = Self {
            id_gen: IdGenerator::new(),
            config: neat_config,
            clients: HashMap::with_capacity(clients_count),
            connection_genes: HashSet::new(),
            species: Vec::new(),
        };
        for _ in 0..clients_count {
            let client = Client::new(neat.id_gen.gen(), Genome::empty(&neat.config));
            neat.clients.insert(client.id, client);
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

    fn gen_species(&mut self) {
        for species in &mut self.species {
            species.reset();
        }
        for client in self.clients.values() {
            if self
                .species
                .iter()
                .any(|species| species.clients.contains(&client.id))
            {
                continue;
            }
            let mut found = false;
            for species in &mut self.species {
                if species.insert(client, &self.clients, &self.config) {
                    found = true;
                    break;
                }
            }
            if !found {
                self.species.push(Species::new(client.id));
            }
        }
        for species in &mut self.species {
            species.evaluate_score(&self.clients);
        }
        self.config.cp += self.config.cp_change_rate
            * (self.species.len() as f32 - self.config.target_species_count as f32);
        self.config.cp = self.config.cp.max(0.1);
    }

    fn kill(&mut self) {
        for species in &mut self.species {
            species.kill(&self.clients, 1.0 - self.config.survivors_percentage);
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
        let ids: Vec<Id> = self.clients.keys().copied().collect();
        for id in ids {
            if self
                .species
                .iter()
                .any(|species| species.clients.contains(&id))
            {
                continue;
            }
            let mut random = rand::thread_rng();
            if let Ok(species) = self
                .species
                .choose_weighted_mut(&mut random, |species| species.score)
            {
                let genome = species.breed(&self.clients, &self.config);
                let client = self.clients.get_mut(&id).unwrap();
                client.genome = genome;
                species.insert_force(client);
            }
        }
    }

    fn mutate(&mut self) {
        let mut ids: Vec<Id> = self.clients.keys().copied().collect();
        ids.sort_by(|a, b| {
            self.clients
                .get(a)
                .unwrap()
                .score
                .partial_cmp(&self.clients.get(b).unwrap().score)
                .unwrap()
        });
        let skip = (self.clients.len() as f32 * (1.0 - self.config.clients_mutation_rate)) as usize;
        for id in ids.into_iter().skip(skip) {
            self.clients
                .get_mut(&id)
                .unwrap()
                .genome
                .mutate(&self.config, &mut self.connection_genes);
        }
    }

    pub(crate) fn get_connection_gene(
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
            *connection
        } else {
            let connection = ConnectionGene::new(Gene::new(), node_from, node_to, weight, enabled);
            connection_genes.insert(connection);
            connection
        }
    }
}
