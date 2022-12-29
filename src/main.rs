mod component;
mod logic_gate;

use component::{ComponentComposer, PinAddr};
use logic_gate::LogicGate;

fn main() {
    let and = LogicGate::or("and", 2);
    let not = LogicGate::not("not");

    let mut comp = ComponentComposer::new("comp")
        .add_comp(and)
        .add_comp(not)
        .connect(PinAddr::new("and", 0), PinAddr::new("not", 0))
        .compose();

    comp.set_ins(vec![false, false]).update();
    println!("{:?}", comp.outs);
}
