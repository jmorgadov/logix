lalrpop_mod!(pub grammar);

mod ast;
mod builder;

use lalrpop_util::lalrpop_mod;
use logix_sim::{flattener::FlattenComponent, Simulation};

fn main() {
    //  Get file from args
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];

    // Get file text
    let text = std::fs::read_to_string(file).unwrap();
    println!("{}", text);

    // Remove new lines
    let text = text.replace("\n", "");

    let comp = grammar::CircuitParser::new().parse(&text).unwrap();

    // println!("{:?}", comp);

    let comp = builder::circuit_to_comp(comp);

    // println!("{:?}", comp);

    let flat = FlattenComponent::new(comp);
    // println!("{:?}", flat);
    let mut sim = Simulation::new(flat);
    sim.start();
}
