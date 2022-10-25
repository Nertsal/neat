use super::*;

#[derive(Debug, PartialEq)]
pub struct Client {
    pub id: Id,
    pub genome: Genome,
    pub score: f32,
}

impl Client {
    pub fn new(id: Id, genome: Genome) -> Self {
        Self {
            id,
            genome,
            score: 0.0,
        }
    }
    pub fn calculate(&self, input: Vec<f32>) -> Vec<f32> {
        self.genome.calculate(input)
    }
}
