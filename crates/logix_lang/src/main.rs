mod ast;
mod builder;

use std::{collections::HashMap, path::Path};

use log::error;
use logix_sim::{
    flatten::{FlattenComponent, NestedConfig},
    primitives::bit::Bit,
    simulator::Simulator,
};

fn show_full(comp: &FlattenComponent, id_to_name: &HashMap<usize, String>) {
    _show_full(comp, &comp.nested_config, &id_to_name, 0);
}

fn _show_full(
    comp: &FlattenComponent,
    config: &NestedConfig,
    id_to_name: &HashMap<usize, String>,
    level: usize,
) {
    match config {
        NestedConfig::Single(idx) => {
            let _comp = &comp.components[*idx];
            println!(
                "{}[{}]{}: {} {}",
                "  ".repeat(level),
                idx,
                id_to_name[&_comp.id],
                Bit::show_vec(&_comp.inputs),
                Bit::show_vec(&_comp.outputs)
            );
        }
        NestedConfig::Compose(id, comps, ins, outs) => {
            let inputs = ins
                .iter()
                .map(|(c_idx, p_idx)| comp.components[*c_idx].inputs[*p_idx])
                .collect::<Vec<_>>();

            let outputs = outs
                .iter()
                .map(|(c_idx, p_idx)| comp.components[*c_idx].outputs[*p_idx])
                .collect::<Vec<_>>();

            println!(
                "{}{}: {} {}",
                "  ".repeat(level),
                id_to_name[id],
                Bit::show_vec(&inputs),
                Bit::show_vec(&outputs)
            );
            for (_, nested) in comps {
                _show_full(comp, nested, id_to_name, level + 1);
            }
        }
    }
}

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

    let (comp, id_map) = match builder::build_from_file(path_str) {
        Ok(comp) => comp,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    println!("ID Map: {:?}", id_map);
    println!("Component: {:?}", comp);

    let flat = match FlattenComponent::new(comp) {
        Ok(flat) => flat,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    println!("Flattened component: {:?}", flat);

    let sim = Simulator::new(
        flat,
        Box::new(move |flat_comp, stats| {
            let delta_ms = stats.upd_time_ns as f64 / 1_000_000.0;
            let loops_per_sec = 1_000.0 / delta_ms;
            let last_cycle_delta = stats.cycle_time_ns as f64 / 1_000_000.0;
            let cycles_per_sec = 1_000.0 / last_cycle_delta;

            print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            println!("Upd time: {}ms", delta_ms);
            println!("Upds/sec: {}", loops_per_sec);
            println!("Cycle time: {}ms", last_cycle_delta);
            println!("Cycles/sec: {}", cycles_per_sec);

            show_full(flat_comp, &id_map);
        }),
    );
    sim.start();
}
