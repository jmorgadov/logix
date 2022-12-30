mod components;
mod id_factory;

use components::{
    clock::Clock,
    component::{Component, ComponentComposer, PinAddr, SimEvent},
    logic_gate::LogicGate,
    pin::Pin,
};
use id_factory::IDFactory;
use std::time::Instant;

fn main() {
    let mut id = IDFactory::new();
    let clock = Clock::new(id.from("clock"), 1.0);
    let clock2 = Clock::new(id.from("clock2"), 4.0);
    let nor1 = LogicGate::nor(id.from("nor1"), 2);
    let nor2 = LogicGate::nor(id.from("nor2"), 2);

    let flip_flop = ComponentComposer::new()
        .name("FlipFlop")
        .id(id.from("flip_flop"))
        .add_comp(Box::new(nor1))
        .add_comp(Box::new(nor2))
        .add_comp(Box::new(Pin::input(id.from("i1"))))
        .add_comp(Box::new(Pin::input(id.from("i2"))))
        .add_comp(Box::new(Pin::output(id.from("o1"))))
        .add_comp(Box::new(Pin::output(id.from("o2"))))
        .connect(pin!(id.get("i1"), 0), pin!(id.get("nor1"), 0))
        .connect(pin!(id.get("i2"), 0), pin!(id.get("nor2"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("nor2"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("nor1"), 1))
        .connect(pin!(id.get("nor1"), 0), pin!(id.get("o1"), 0))
        .connect(pin!(id.get("nor2"), 0), pin!(id.get("o2"), 0))
        .compose();

    let mut comp = ComponentComposer::new()
        .name("Main")
        .id(id.from("main"))
        .add_comp(Box::new(clock))
        .add_comp(Box::new(clock2))
        .add_comp(Box::new(flip_flop))
        .add_comp(Box::new(Pin::output(id.from("o3"))))
        .add_comp(Box::new(Pin::output(id.from("o4"))))
        .connect(pin!(id.get("clock"), 0), pin!(id.get("flip_flop"), 0))
        .connect(pin!(id.get("clock2"), 0), pin!(id.get("flip_flop"), 1))
        .connect(pin!(id.get("flip_flop"), 0), pin!(id.get("o3"), 0))
        .connect(pin!(id.get("flip_flop"), 1), pin!(id.get("o4"), 0))
        .compose();

    let start = Instant::now();
    loop {
        let time = start.elapsed().as_nanos();
        comp.on_event(&SimEvent::Update(time));
        if comp.is_dirty() {
            comp.check_values();
            println!("{:?}", comp.outs());
        }
    }
}
