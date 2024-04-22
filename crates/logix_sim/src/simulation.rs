use crate::{bit::*, flattener::FlattenComponent, primitives::prelude::*};
use log::debug;
use logix_core::prelude::*;
use std::time::Instant;

/// Simulation.
pub struct Simulation {
    comp: FlattenComponent,
    running: bool,

    upd_list: Vec<usize>,
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
            upd_list: upd_queue,
            needs_update: needs_update,
        }
    }

    /// Starts the simulation.
    pub fn start(&mut self) {
        self.running = true;

        let start = Instant::now();
        while self.upd_list.len() > 0 {
            let time = start.elapsed().as_nanos();

            debug!("Update list: {:?}", self.upd_list);

            let rand_idx = rand::random::<usize>() % self.upd_list.len();
            let comp_idx = self.upd_list[rand_idx];
            let comp = &mut self.comp.components[comp_idx];

            debug!("Updating component: {:?}", comp.name);
            debug!("  Old outputs: {:?}", comp.outputs);

            let c_type = &self.comp.c_types[comp_idx];
            update_comp(comp, c_type, time);

            debug!("  New outputs: {:?}", comp.outputs);

            if *c_type != Primitive::Clock {
                self.upd_list.remove(rand_idx);
                self.needs_update[comp_idx] = false;
            }

            for conn in self
                .comp
                .connections
                .iter()
                .filter(|conn| conn.from.0 == comp_idx)
            {
                let val = self.comp.components[comp_idx].outputs[conn.from.1];

                // Do not update if the value is the same
                if val == self.comp.components[conn.to.0].inputs[conn.to.1] {
                    continue;
                }

                debug!(
                    "New comp to update: {:?}",
                    self.comp.components[conn.to.0].name
                );

                // Update the value
                self.comp.components[conn.to.0].inputs[conn.to.1] = val;
                if !self.needs_update[conn.to.0] {
                    self.upd_list.push(conn.to.0);
                    self.needs_update[conn.to.0] = true;
                }
            }

            let time2 = start.elapsed().as_nanos();

            let delta_ms = (time2 - time) as f64 / 1_000_000.0;
            let loops_per_sec = 1_000.0 / delta_ms;

            print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            println!("Time: {}ms - Loops per second: {}", delta_ms, loops_per_sec);
            self.comp.show();
        }
    }
}

fn update_comp(comp: &mut Component<Bit, BaseExtra>, c_type: &Primitive, time: u128) {
    match c_type {
        Primitive::NotGate => comp.outputs[0] = !comp.inputs[0],
        Primitive::AndGate => {
            comp.outputs[0] = comp.inputs.iter().fold(true, |acc, f| acc & f);
        }
        Primitive::NandGate => {
            comp.outputs[0] = !comp.inputs.iter().fold(true, |acc, f| acc & f);
        }
        Primitive::OrGate => {
            comp.outputs[0] = comp.inputs.iter().fold(false, |acc, f| acc | f);
        }
        Primitive::NorGate => {
            comp.outputs[0] = !comp.inputs.iter().fold(false, |acc, f| acc | f);
        }
        Primitive::XorGate => {
            comp.outputs[0] = comp.inputs.iter().fold(false, |acc, f| acc ^ f);
        }
        // No update needed
        Primitive::Clock => {
            if let BaseExtra::Clock(frec) = comp.extra {
                let val = (time % (frec * 2)) > frec;
                comp.outputs[0] = val;
            } else {
                panic!("Clock component without frec information");
            }
        }
        Primitive::HighConst => (),
        Primitive::LowConst => (),
        Primitive::Unknown => unreachable!(),
    }
}
