use super::*;

pub struct Client {
    pub genome: Genome,
    pub calculator: Calculator,
    pub species: Option<Species>,
    pub score: f32,
}

impl Client {
    pub fn new(genome: Genome) -> Self {
        let calculator = Calculator::new(&genome);
        Self {
            genome,
            calculator,
            species: None,
            score: 0.0,
        }
    }
    pub fn calculate(&self, input: Vec<f32>) -> Vec<f32> {
        self.calculator.calculate(input)
    }
}
