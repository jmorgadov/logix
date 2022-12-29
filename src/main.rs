mod components;

use components::{
    clock::Clock,
    component::{Component, ComponentComposer, PinAddr},
    logic_gate::LogicGate,
};
use std::time::Instant;

fn main() {
    let clock = Clock::new(10, 1000);
    let and = LogicGate::and(0, 2);
    let not = LogicGate::not(1);

    let mut comp = ComponentComposer::new(2)
        .add_comp(Box::new(clock))
        .add_comp(Box::new(and))
        .add_comp(Box::new(not))
        .connect(PinAddr::new(10, 0), PinAddr::new(0, 0))
        .connect(PinAddr::new(0, 0), PinAddr::new(1, 0))
        .compose();

    comp.set_ins(vec![true]);
    let start = Instant::now();
    loop {
        let time = start.elapsed().as_millis();
        comp.update(time);
        if comp.is_dirty() {
            comp.check_values();
            println!("{:?}", comp.outs());
        }
    }
}
