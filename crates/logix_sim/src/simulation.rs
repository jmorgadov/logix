use crate::{bit::*, flattener::FlattenComponent, primitives::prelude::*};
use logix_core::prelude::*;
use std::time::Instant;

/// Simulation.
pub struct Simulation {
    comp: FlattenComponent,
    running: bool,
}

impl Simulation {
    pub fn new(comp: FlattenComponent) -> Self {
        Simulation {
            comp,
            running: false,
        }
    }

    /// Starts the simulation.
    pub fn start(&mut self) {
        self.running = true;

        let start = Instant::now();
        while self.running {
            let time = start.elapsed().as_nanos();
            let dirty = update_time(&mut self.comp, time);
            if dirty {
                update_values(&mut self.comp);
                print!("{}[2J", 27 as char);
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                self.comp.show();
            }
        }
    }
}


fn update_time(main: &mut FlattenComponent, time: u128) -> bool {
    let mut dirty = false;
    for comp in main.components.iter_mut() {
        if let Primitive::Clock = Primitive::from_name(&comp.name) {
            let interv =
                u128::from_ne_bytes(comp.info.as_slice().try_into().expect("Wrong clock info"));
            let val = (time % (interv * 2)) > interv;
            let dirty_clock = comp.outputs[0] != val.into();
            comp.outputs[0] = val.into();
            dirty |= dirty_clock;
        }
    }
    dirty
}

fn update_comp(comp: &mut Component<Bit>) {
    match Primitive::from_name(&comp.name) {
        Primitive::NotGate => comp.outputs[0] = !comp.inputs[0],
        Primitive::AndGate => {
            let mut out = comp.inputs[0];
            for bit in &comp.inputs[1..comp.inputs.len()] {
                out = out & *bit;
                if out == ZERO {
                    break;
                }
            }
            comp.outputs[0] = out;
        }
        Primitive::NandGate => {
            let mut out = comp.inputs[0];
            for bit in &comp.inputs[1..comp.inputs.len()] {
                out = out & *bit;
            }
            comp.outputs[0] = !out;
        }
        Primitive::OrGate => {
            let mut out = comp.inputs[0];
            for bit in &comp.inputs[1..comp.inputs.len()] {
                out = out | *bit;
            }
            comp.outputs[0] = out;
        }
        Primitive::NorGate => {
            let mut out = comp.inputs[0];
            for bit in &comp.inputs[1..comp.inputs.len()] {
                out = out | *bit;
            }
            comp.outputs[0] = !out;
        }
        Primitive::XorGate => {
            let mut out = comp.inputs[0];
            for i in 1..comp.inputs.len() {
                out = out ^ comp.inputs[i];
                if out == UNK || out == ONE {
                    break;
                }
            }
            comp.inputs[0] = out;
        }
        // No update needed
        Primitive::Clock => (),
        Primitive::HighConst => (),
        Primitive::LowConst => (),
        Primitive::Unknown => panic!("Unreashable"),
    }
}

fn update_values(main: &mut FlattenComponent) {
    let mut visit_idx = 0;
    let mut visit = vec![0; main.components.len()];
    let mut stack: Vec<usize> = vec![];

    while !stack.is_empty() || visit_idx < visit.len() {
        if stack.is_empty() {
            if visit[visit_idx] != 0 {
                visit_idx += 1;
                continue;
            }
            stack.push(visit_idx);
        }

        let idx = *stack.last().unwrap();

        let mut ready_to_update = true;

        for dep in &main.deps[idx] {
            if visit[*dep] == 0 {
                stack.push(*dep);
                visit[*dep] = 1;
                ready_to_update = false;
            }
        }

        if ready_to_update {
            update_comp(&mut main.components[idx]);

            for conn in &main.connections {
                if idx_of(conn.from) == idx {
                    let val = main.components[idx].outputs[addr_of(conn.from)];
                    main.components[idx_of(conn.to)].inputs[addr_of(conn.to)] = val;
                }
            }

            visit[idx] = 2;
            stack.pop();
        }
    }
}
