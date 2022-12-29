mod component;
mod logic_gate;

use component::{ComponentComposer, PinAddr};
use logic_gate::LogicGate;

fn main() {
    let and = LogicGate::or(0, 2);
    let not = LogicGate::not(1);

    let mut comp = ComponentComposer::new(2)
        .add_comp(and)
        .add_comp(not)
        .connect(PinAddr::new(0, 0), PinAddr::new(1, 0))
        .compose();

    comp.set_ins(vec![false, false]).update();
    println!("{:?}", comp.outs);
}
