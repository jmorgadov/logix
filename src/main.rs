use std::time::Instant;

use components::{
    component::{Component, SimEvent},
    composed_component::{ComposedComponentBuilder, PinAddr},
    primitives::{clock::Clock, input_pin::InputPin, nor_gate::NorGate, output_pin::OutputPin},
};
use id_factory::IDFactory;

mod components;
mod id_factory;

fn main() {
    let mut id = IDFactory::new();
    let clock1 = Clock::new(id.from("clock1"), 1.0);
    let clock2 = Clock::new(id.from("clock2"), 4.0);
    let nor1 = NorGate::new(id.from("nor1"), 2);
    let nor2 = NorGate::new(id.from("nor2"), 2);

    let sr_latch = ComposedComponentBuilder::new()
        .name("SRLatch")
        .id(id.from("sr_latch"))
        .add_comp(Box::new(InputPin::new(id.from("i1"))))
        .add_comp(Box::new(InputPin::new(id.from("i2"))))
        .add_comp(Box::new(nor1))
        .add_comp(Box::new(nor2))
        .add_comp(Box::new(OutputPin::new(id.from("o1"))))
        .add_comp(Box::new(OutputPin::new(id.from("o2"))))
        .connect(pin!(id.get("i1"), 0), pin!(id.get("nor1"), 0))
        .connect(pin!(id.get("i2"), 0), pin!(id.get("nor2"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("nor2"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("nor1"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("o1"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("o2"), 0))
        .build();

    let mut comp = ComposedComponentBuilder::new()
        .name("Main")
        .id(id.from("main"))
        .add_comp(Box::new(clock1))
        .add_comp(Box::new(clock2))
        .add_comp(Box::new(sr_latch))
        .add_comp(Box::new(OutputPin::new(id.from("o3"))))
        .add_comp(Box::new(OutputPin::new(id.from("o4"))))
        .connect(pin!(id.get("clock1"), 0), pin!(id.get("sr_latch"), 0))
        .connect(pin!(id.get("clock2"), 0), pin!(id.get("sr_latch"), 1))
        .connect(pin!(id.get("sr_latch"), 0), pin!(id.get("o3"), 0))
        .connect(pin!(id.get("sr_latch"), 1), pin!(id.get("o4"), 0))
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
