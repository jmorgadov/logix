use crate::{bit::*, flattener::FlattenComponent, primitives::prelude::*};
use logix_core::prelude::*;
use std::time::Instant;

/// Simulation.
pub struct Simulation {
    comp: FlattenComponent,
    running: bool,

    upd_queue: Vec<usize>,
    needs_update: Vec<bool>,
}

impl Simulation {
    pub fn new(comp: FlattenComponent) -> Self {
        let count = comp.components.len();

        let upd_queue = (0..count).collect::<Vec<usize>>();
        let needs_update = vec![true; count];
        Simulation {
            comp,
            running: false,
            upd_queue: upd_queue,
            needs_update: needs_update,
        }
    }

    /// Starts the simulation.
    pub fn start(&mut self) {
        self.running = true;

        let start = Instant::now();
        while self.upd_queue.len() > 0 {
            println!("Comp to upd: {:?}", self.upd_queue.iter().map(|x| self.comp.components[*x].name.to_string()).collect::<Vec<String>>());
            let time = start.elapsed().as_nanos();

            let rand_idx = rand::random::<usize>() % self.upd_queue.len();
            let comp_idx = self.upd_queue[rand_idx];
            let comp = &mut self.comp.components[comp_idx];
            let c_type = &self.comp.c_types[comp_idx];
            update_comp(comp, c_type, time);

            if *c_type != Primitive::Clock {
                self.upd_queue.remove(rand_idx);
                self.needs_update[comp_idx] = false;
            }

            for conn in self
                .comp
                .connections
                .iter()
                .filter(|conn| conn.from.0 == comp_idx)
            {
                let val = self.comp.components[comp_idx].outputs[conn.from.1];
                println!("Val: {:?}", val);

                // Do not update if the value is the same
                if val == self.comp.components[conn.to.0].inputs[conn.to.1] {
                    continue;
                }

                // Update the value
                self.comp.components[conn.to.0].inputs[conn.to.1] = val;
                if !self.needs_update[conn.to.0] {
                    self.upd_queue.push(conn.to.0);
                    self.needs_update[conn.to.0] = true;
                }
            }

            print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            self.comp.show();
        }
    }
}

fn update_comp(comp: &mut Component<Bit>, c_type: &Primitive, time: u128) {
    match c_type {
        Primitive::NotGate => comp.outputs[0] = !comp.inputs[0],
        Primitive::AndGate => {
            let mut out = comp.inputs[0];
            for bit in &comp.inputs[1..comp.inputs.len()] {
                out = out & *bit;
                if !out {
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
            for bit in &comp.inputs[1..comp.inputs.len()] {
                out = out ^ *bit;
            }
            comp.inputs[0] = out;
        }
        // No update needed
        Primitive::Clock => {
            let interv =
                u128::from_ne_bytes(comp.info.as_slice().try_into().expect("Wrong clock info"));
            let val = (time % (interv * 2)) > interv;
            comp.outputs[0] = val;
        }
        Primitive::HighConst => (),
        Primitive::LowConst => (),
        Primitive::Unknown => panic!("Unreashable"),
    }
}
