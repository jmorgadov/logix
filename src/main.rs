use components::{prelude::*, primitives::prelude::*};
use simulation::Simulation;

mod components;
mod json_serialization;
mod parser;
mod simulation;
mod visitor;

fn main() {
    let sr_latch = ComposedComponentBuilder::new("SRLatch")
        .add_comp(0, Box::new(InputPin::new(0)))
        .add_comp(1, Box::new(InputPin::new(1)))
        .add_comp(2, Box::new(NorGate::new(2)))
        .add_comp(3, Box::new(NorGate::new(2)))
        .add_comp(4, Box::new(OutputPin::new(0)))
        .add_comp(5, Box::new(OutputPin::new(1)))
        .connect(pin!(0, 0), pin!(2, 0))
        .connect(pin!(1, 0), pin!(3, 1))
        .connect(pin!(2, 0), pin!(3, 0))
        .connect(pin!(3, 0), pin!(2, 1))
        .connect(pin!(2, 0), pin!(4, 0))
        .connect(pin!(3, 0), pin!(5, 0))
        .build();

    let comp = ComposedComponentBuilder::new("Main")
        .add_comp(0, Box::new(Clock::new(1.0)))
        .add_comp(1, Box::new(Clock::new(4.0)))
        .add_comp(2, Box::new(sr_latch))
        .add_comp(3, Box::new(OutputPin::new(0)))
        .add_comp(4, Box::new(OutputPin::new(1)))
        .connect(pin!(0, 0), pin!(2, 0))
        .connect(pin!(1, 0), pin!(2, 1))
        .connect(pin!(2, 0), pin!(3, 0))
        .connect(pin!(2, 1), pin!(4, 0))
        .build();

    let mut simulation = Simulation::new(Box::new(comp));
    simulation.start();
}
