use std::time::Instant;

use components::{
    component::{Component, SimEvent},
    compose_component::{ComposedComponentBuilder, PinAddr},
    primitives::{clock::Clock, nor_gate::NorGate, output_pin::OutputPin},
};
use id_factory::IDFactory;

mod components;
mod id_factory;

fn main() {
    let mut id = IDFactory::new();
    let clock = Clock::new(id.from("clock"), 1.0);
    let clock2 = Clock::new(id.from("clock2"), 4.0);
    let nor1 = NorGate::new(id.from("nor1"), 2);
    let nor2 = NorGate::new(id.from("nor2"), 2);

    let mut comp = ComposedComponentBuilder::new()
        .name("FlipFlop")
        .id(id.from("flip_flop"))
        .add_comp(Box::new(clock))
        .add_comp(Box::new(clock2))
        .add_comp(Box::new(nor1))
        .add_comp(Box::new(nor2))
        .add_comp(Box::new(OutputPin::new(id.from("o1"))))
        .add_comp(Box::new(OutputPin::new(id.from("o2"))))
        .connect(pin!(id.get("clock"), 0), pin!(id.get("nor1"), 0))
        .connect(pin!(id.get("clock2"), 0), pin!(id.get("nor2"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("nor2"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("nor1"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("o1"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("o2"), 0))
        .build();

    let start = Instant::now();
    loop {
        let time = start.elapsed().as_nanos();
        comp.on_event(&SimEvent::Update(time));
        if comp.is_dirty() {
            comp.on_event(&SimEvent::UpdateValues);
            println!("{:?}", comp.outs());
        }
    }
}
