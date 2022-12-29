mod components;
mod id_factory;

use components::{
    clock::Clock,
    component::{Component, ComponentComposer, PinAddr},
    logic_gate::LogicGate,
};
use id_factory::IDFactory;
use std::time::Instant;

fn main() {
    let mut id = IDFactory::new();
    let clock = Clock::new(id.from("clock"), 1.0);
    let and = LogicGate::and(id.from("and"), 2);
    let not = LogicGate::not(id.from("not"));

    let mut comp = ComponentComposer::new(id.from("comp"))
        .add_comp(Box::new(clock))
        .add_comp(Box::new(and))
        .add_comp(Box::new(not))
        .connect(pin!(id.get("clock"), 0), pin!(id.get("and"), 0))
        .connect(pin!(id.get("and"), 0), pin!(id.get("not"), 0))
        .compose();

    comp.set_ins(vec![true]);
    let start = Instant::now();
    loop {
        let time = start.elapsed().as_nanos();
        comp.update(time);
        if comp.is_dirty() {
            comp.check_values();
            println!("{:?}", comp.outs());
        }
    }
}
