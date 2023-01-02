use std::time::Instant;

use components::{prelude::*, primitives::prelude::*};
use id_factory::IDFactory;
use simulation::Simulation;

mod components;
mod id_factory;
mod serialize;
mod simulation;

fn main() {
    let mut id = IDFactory::new();

    let sr_latch = ComposedComponentBuilder::new()
        .name("SRLatch")
        .id(id.set("sr_latch"))
        .add_comp(Box::new(InputPin::new(id.set("i1"))))
        .add_comp(Box::new(InputPin::new(id.set("i2"))))
        .add_comp(Box::new(NorGate::new(id.set("nor1"), 2)))
        .add_comp(Box::new(NorGate::new(id.set("nor2"), 2)))
        .add_comp(Box::new(OutputPin::new(id.set("o1"))))
        .add_comp(Box::new(OutputPin::new(id.set("o2"))))
        .connect(pin!(id.get("i1"), 0), pin!(id.get("nor1"), 0))
        .connect(pin!(id.get("i2"), 0), pin!(id.get("nor2"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("nor2"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("nor1"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("o1"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("o2"), 0))
        .build();

    let comp = ComposedComponentBuilder::new()
        .name("Main")
        .id(id.set("main"))
        .add_comp(Box::new(Clock::new(id.set("clock1"), 1.0)))
        .add_comp(Box::new(Clock::new(id.set("clock2"), 4.0)))
        .add_comp(Box::new(sr_latch))
        .add_comp(Box::new(OutputPin::new(id.set("o3"))))
        .add_comp(Box::new(OutputPin::new(id.set("o4"))))
        .connect(pin!(id.get("clock1"), 0), pin!(id.get("sr_latch"), 0))
        .connect(pin!(id.get("clock2"), 0), pin!(id.get("sr_latch"), 1))
        .connect(pin!(id.get("sr_latch"), 0), pin!(id.get("o3"), 0))
        .connect(pin!(id.get("sr_latch"), 1), pin!(id.get("o4"), 0))
        .build();

    let mut simulation = Simulation::new(Box::new(comp));
    simulation.start();
}
