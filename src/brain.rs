use bevy::ecs::{
    component::Component,
};
use rand::{distributions::{Distribution, Uniform}, rngs::ThreadRng, Rng};
const INPUT_SIZE: usize = 3;
const OUTPUT_SIZE: usize = 2;

#[derive(Copy, Clone)]
enum NeuronType {
    Input(usize),
    Internal(usize),
    Output(usize),
}
impl NeuronType {
    pub fn unwrap(self) -> usize {
        match self {
            NeuronType::Input(x) |
            NeuronType::Internal(x) |
            NeuronType::Output(x) => {
                return x
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum MutationEvent {
    MutWeight,
    AddConnection,
    RemoveConnection,
    None
}
impl MutationEvent {
    pub fn random() -> MutationEvent {
        use MutationEvent::*;
        let mut rng = rand::thread_rng();
        let weight = vec![MutWeight; 5];
        let none = vec![None; 93];
        let connection = vec![AddConnection, RemoveConnection];
        let dist: Vec<MutationEvent> = weight.into_iter().chain(none.into_iter()).chain(connection.into_iter()).collect();
        let range = Uniform::from(0..100);
        dist[range.sample(&mut rng)]
    }
}
#[derive(Component, Clone)]
pub struct Brain {
    pub inputs: Vec<f32>,
    pub outputs: Vec<f32>,
    pub synapses: Vec<Synapse>
}
#[derive(Clone)]
pub struct Synapse {
    from: NeuronType,
    to: NeuronType,
    weight: f32,
    enabled: bool,
}
impl Brain {
    pub fn new() -> Brain {
        Brain {
            inputs: vec![0.0; INPUT_SIZE],
            outputs: vec![0.0; OUTPUT_SIZE],
            synapses: vec![]
        }
    }
    pub fn from(brain: Brain) -> Brain {
        let mut rng = rand::thread_rng();
        let mut new_brain = brain.clone();
        let mutation_event = MutationEvent::random();
        match mutation_event {
            MutationEvent::MutWeight => {
                let range = Uniform::from(0..brain.synapses.len());
                new_brain.synapses[range.sample(&mut rng)].weight += if rng.gen::<bool>() { 0.1 } else { -0.1 };
            },
            MutationEvent::AddConnection => {
                new_brain.create_synapse();
            },
            MutationEvent::RemoveConnection => {
                let range = Uniform::from(0..brain.synapses.len());
                new_brain.synapses.remove(range.sample(&mut rng));
            },
            MutationEvent::None => {}
        };
        new_brain
    }
    pub fn create_synapse(&mut self){
        let mut rng = rand::thread_rng();
        let i_range = Uniform::from(0..INPUT_SIZE);
        let o_range = Uniform::from(0..OUTPUT_SIZE);

        let from = NeuronType::Input(i_range.sample(&mut rng));
        let to = NeuronType::Output(o_range.sample(&mut rng));
        let weight = rng.gen::<f32>();
        let synapse = Synapse {
            from,
            to,
            weight,
            enabled: true,
        };
        self.synapses.push(synapse);
    }
    pub fn evaluate(&mut self){
        let mut out = vec![0.0; OUTPUT_SIZE];
        for syn in &self.synapses {

        }
        self.outputs = out;
    }
    pub fn update_inputs(&mut self, v: Vec<f32>){
        self.inputs = v;
    }
}
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}
