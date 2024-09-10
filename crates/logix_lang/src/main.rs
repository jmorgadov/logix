mod ast;
mod builder;

use std::{collections::HashMap, path::Path, thread};

use log::error;
use logix_sim::{
    flatten::{FlattenComponent, NestedConfig},
    primitives::data::Data,
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
        NestedConfig::Single(_, idx, _, _) => {
            let _comp = &comp.components[*idx];
            println!(
                "{}[{}]{}: {} {}",
                "  ".repeat(level),
                idx,
                id_to_name[&_comp.id],
                Data::show_vec(&_comp.inputs),
                Data::show_vec(&_comp.outputs)
            );
        }
        NestedConfig::Compose(_, id, comps, ins, outs) => {
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
                Data::show_vec(&inputs),
                Data::show_vec(&outputs)
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

    let mut sim = Simulator::new(flat);
    sim.start(false);
    loop {
        sim.state(|state| {
            print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            show_full(&state.comp, &id_map);
        });
        thread::sleep(std::time::Duration::from_millis(5));
    }
}
