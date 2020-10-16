use super::*;

#[derive(Debug)]
pub struct Client {
    pub genome: Genome,
    pub species: Option<Species>,
    pub score: f32,
}

impl Client {
    pub fn new(genome: Genome) -> Self {
        Self {
            genome,
            species: None,
            score: 0.0,
        }
    }
    pub fn calculate(&self, input: Vec<f32>) -> Vec<f32> {
        self.genome.calculate(input)
    }
}
