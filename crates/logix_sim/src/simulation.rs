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

    on_upd: Box<dyn Fn(&FlattenComponent, &SimStats)>,
}

pub struct SimStats {
    pub upd_time_ns: u128,
    pub cycle_time_ns: u128,
}

impl Simulation {
    pub fn new(comp: FlattenComponent, on_upd: Box<dyn Fn(&FlattenComponent, &SimStats)>) -> Self {
        let count = comp.components.len();

        let upd_queue = (0..count).collect::<Vec<usize>>();
        let needs_update = vec![true; count];

        Simulation {
            comp,
            running: false,
            upd_list: upd_queue,
            needs_update: needs_update,
            on_upd,
        }
    }

    /// Starts the simulation.
    pub fn start(&mut self) {
        self.running = true;

        let start = Instant::now();
        let mut non_clocks_to_upd_count = self
            .comp
            .components
            .iter()
            .filter(|c| c.name != "Clock")
            .count();

        let mut stats = SimStats {
            upd_time_ns: 0,
            cycle_time_ns: 0,
        };
        let mut last_cycle_upd = start;

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
                non_clocks_to_upd_count -= 1;
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
                    non_clocks_to_upd_count += 1;
                }
            }

            if non_clocks_to_upd_count == 0 {
                stats.cycle_time_ns = last_cycle_upd.elapsed().as_nanos();
                last_cycle_upd = Instant::now();
            }

            let time2 = start.elapsed().as_nanos();
            stats.upd_time_ns = time2 - time;

            (self.on_upd)(&self.comp, &stats);
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
