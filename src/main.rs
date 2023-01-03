use components::{prelude::*, primitives::prelude::*};
use simulation::Simulation;

mod components;
mod json_serialization;
mod parser;
mod simulation;
mod visitor;

fn main() {
    let sr_latch = ComposedComponentBuilder::new("SRLatch")
        .components(vec![Box::new(NorGate::new(2)), Box::new(NorGate::new(2))])
        .connections(vec![conn!((0, 0), (1, 0)), conn!((1, 0), (0, 1))])
        .inputs(vec![(0, 0), (1, 1)])
        .outputs(vec![(0, 0), (1, 0)])
        .build()
        .unwrap();

    let comp = ComposedComponentBuilder::new("Main")
        .components(vec![
            Box::new(Clock::new(1.0)),
            Box::new(Clock::new(4.0)),
            Box::new(sr_latch),
        ])
        .connections(vec![conn!((0, 0), (2, 0)), conn!((1, 0), (2, 1))])
        .outputs(vec![(2, 0), (2, 1)])
        .build()
        .unwrap();

    json_serialization::save("example.json", &comp);
    let loaded = json_serialization::load("example.json").unwrap();
    let mut simulation = Simulation::new(Box::new(loaded));
    simulation.start();
}
