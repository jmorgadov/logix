use crate::{flatten::FlattenComponent, primitives::primitives::Primitive};
use log::debug;
use std::time::Instant;

/// Simulation.
pub struct Simulator {
    comp: FlattenComponent,
    running: bool,

    upd_list: Vec<usize>,
    needs_update: Vec<bool>,

    on_upd: Box<dyn FnMut(&FlattenComponent, &SimStats)>,
}

pub struct SimStats {
    pub upd_time_ns: u128,
    pub cycle_time_ns: u128,
    pub end_cycle: bool,
}

impl Simulator {
    pub fn new(comp: FlattenComponent, on_upd: Box<dyn FnMut(&FlattenComponent, &SimStats)>) -> Self {
        let count = comp.components.len();

        let upd_queue = (0..count).collect::<Vec<usize>>();
        let needs_update = vec![true; count];

        Simulator {
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
            .filter(|c| {
                if let Primitive::Clock { period: _p } = c.prim_type {
                    false
                } else {
                    true
                }
            })
            .count();

        let mut stats = SimStats {
            upd_time_ns: 0,
            cycle_time_ns: 0,
            end_cycle: false,
        };
        let mut last_cycle_upd = start;

        while self.upd_list.len() > 0 {
            let time = start.elapsed().as_nanos();

            debug!("Update list: {:?}", self.upd_list);

            let rand_idx = rand::random::<usize>() % self.upd_list.len();
            let comp_idx = self.upd_list[rand_idx];
            let comp = &mut self.comp.components[comp_idx];

            debug!("Updating component: {:?}", comp.prim_type);
            debug!("  Old outputs: {:?}", comp.outputs);

            comp.update(time);

            debug!("  New outputs: {:?}", comp.outputs);

            match comp.prim_type {
                Primitive::Clock { period: _p } => {}
                _ => {
                    self.upd_list.remove(rand_idx);
                    self.needs_update[comp_idx] = false;
                    non_clocks_to_upd_count -= 1;
                }
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
                    self.comp.components[conn.to.0].prim_type
                );

                // Update the value
                self.comp.components[conn.to.0].inputs[conn.to.1] = val;
                if !self.needs_update[conn.to.0] {
                    self.upd_list.push(conn.to.0);
                    self.needs_update[conn.to.0] = true;
                    non_clocks_to_upd_count += 1;
                }
            }

            stats.end_cycle = false;
            if non_clocks_to_upd_count == 0 {
                stats.cycle_time_ns = last_cycle_upd.elapsed().as_nanos();
                last_cycle_upd = Instant::now();
                stats.end_cycle = true;
            }

            let time2 = start.elapsed().as_nanos();
            stats.upd_time_ns = time2 - time;

            (self.on_upd)(&self.comp, &stats);
        }
    }
}