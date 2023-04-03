use std::{
    cell::Cell,
    sync::mpsc::{self, Receiver, Sender},
};

#[derive(Debug, Clone)]
pub enum NeuronCommand {
    Off,
    Adj(f64),
}

#[derive(Debug)]
pub struct Neuron<T> {
    pub weight: Cell<f64>,
    pub inputs: Vec<Sender<T>>,
    pub consumer: Receiver<T>,
}

impl<T> Neuron<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<T>();
        let inputs = vec![tx];

        Neuron {
            inputs,
            weight: Cell::new(0.0),
            consumer: rx,
        }
    }

    pub fn fire()
}

impl Neuron<NeuronCommand> {
}
