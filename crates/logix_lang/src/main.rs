mod ast;
mod builder;

use std::path::Path;

use log::error;
use logix_sim::{flatten::FlattenComponent, simulator::Simulator};

fn main() {
    env_logger::init();

    //  Get file from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: logix_lang <file>");
        return;
    }

    let file = &args[1];
    let path = Path::new(file).canonicalize().unwrap();
    let path_str = path.to_str().unwrap();

    let comp = match builder::build_from_file(path_str) {
        Ok(comp) => comp,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    println!("Component: {:?}", comp);

    let flat = match FlattenComponent::new(comp) {
        Ok(flat) => flat,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    println!("Flattened component: {:?}", flat);

    let mut sim = Simulator::new(
        flat,
        Box::new(move |flat_comp, stats| {
            print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            let delta_ms = stats.upd_time_ns as f64 / 1_000_000.0;
            let loops_per_sec = 1_000.0 / delta_ms;
            let last_cycle_delta = stats.cycle_time_ns as f64 / 1_000_000.0;
            let cycles_per_sec = 1_000.0 / last_cycle_delta;

            println!("Upd time: {}ms", delta_ms);
            println!("Upds/sec: {}", loops_per_sec);
            println!("Cycle time: {}ms", last_cycle_delta);
            println!("Cycles/sec: {}", cycles_per_sec);

            // flat_comp.show_status_of("adder")
        }),
    );
    sim.start();
}
