lalrpop_mod!(pub grammar);

mod ast;
mod builder;

use lalrpop_util::lalrpop_mod;
use log::error;
use logix_sim::{flattener::FlattenComponent, Simulation};

fn main() {
    env_logger::init();

    //  Get file from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: logix_lang <file>");
        return;
    }

    let file = &args[1];

    // Get file text
    let text = match std::fs::read_to_string(file) {
        Ok(text) => text,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    let comp = match grammar::CircuitParser::new().parse(&text) {
        Ok(comp) => comp,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    // println!("{:?}", comp);

    let comp = match builder::circuit_to_comp(comp) {
        Ok(comp) => comp,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    let flat = match FlattenComponent::new(comp) {
        Ok(flat) => flat,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    let mut sim = Simulation::new(flat);
    sim.start();
}
