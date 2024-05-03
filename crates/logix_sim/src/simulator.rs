use crate::{flatten::FlattenComponent, primitives::primitives::Primitive};
use log::debug;
use rand::seq::SliceRandom;
use std::time::Instant;

pub struct Simulator {
    comp: FlattenComponent,
    running: bool,

    to_upd: Vec<usize>,
    on_upd: Box<dyn FnMut(&FlattenComponent, &SimStats)>,
}

pub struct SimStats {
    pub upd_time_ns: u128,
    pub cycle_time_ns: u128,
    pub end_cycle: bool,
}

impl Simulator {
    pub fn new(
        comp: FlattenComponent,
        on_upd: Box<dyn FnMut(&FlattenComponent, &SimStats)>,
    ) -> Self {
        let to_upd = comp
            .components
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                match c.prim_type {
                    Primitive::Const { value: _v } => {
                        return Some(i);
                    }
                    Primitive::Clock { period: _p } => {
                        return Some(i);
                    }
                    _ => {}
                }
                if let Primitive::Clock { period: _p } = c.prim_type {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        Simulator {
            comp,
            running: false,
            to_upd,
            on_upd,
        }
    }

    /// Starts the simulation.
    pub fn start(mut self) {
        self.running = true;

        let start = Instant::now();
        let mut stats = SimStats {
            upd_time_ns: 0,
            cycle_time_ns: 0,
            end_cycle: false,
        };

        let mut local_upd_queue: Vec<usize> = (0..self.comp.components.len()).collect();
        let mut local_next_upd_queue: Vec<usize> = self.to_upd.clone();

        while self.to_upd.len() > 0 {
            let time = start.elapsed().as_nanos();
            debug!("Update list: {:?}", self.to_upd);

            for idx in local_upd_queue.iter() {
                debug!(
                    "To Update: {:?}",
                    local_upd_queue
                        .iter()
                        .map(|x| format!("{} {}", *x, self.comp.components[*x].name.clone()))
                        .collect::<Vec<String>>()
                );

                let comp_idx = *idx;
                let comp = &mut self.comp.components[comp_idx];
                debug!("Updating component: {} {:?}", comp_idx, comp.prim_type);
                debug!("  Old inputs: {:?}", comp.inputs);
                debug!("  Old outputs: {:?}", comp.outputs);

                comp.update(time);

                debug!("  New outputs: {:?}", comp.outputs);

                match comp.prim_type {
                    Primitive::Clock { period: _p } => {}
                    _ => {
                        self.to_upd.retain(|&x| x != comp_idx);
                    }
                }
                //

                let mut rand_conns =
                    (0..self.comp.connections[comp_idx].len()).collect::<Vec<usize>>();

                rand_conns.shuffle(&mut rand::thread_rng());

                for idx in rand_conns.iter() {
                    let conn = self.comp.connections[comp_idx][*idx];
                    let val = self.comp.components[comp_idx].outputs[conn.from.1];

                    // Do not update if the value is the same
                    if val == self.comp.components[conn.to.0].inputs[conn.to.1] {
                        continue;
                    }

                    debug!("  Connection: {:?}", conn);
                    debug!(
                        "New comp to update: {} {:?}",
                        conn.to.0, self.comp.components[conn.to.0].prim_type
                    );

                    // Update the value
                    self.comp.components[conn.to.0].inputs[conn.to.1] = val;

                    if !local_next_upd_queue.contains(&conn.to.0) {
                        local_next_upd_queue.push(conn.to.0);
                    }
                }

                let time2 = start.elapsed().as_nanos();
                stats.upd_time_ns = time2 - time;

                (self.on_upd)(&self.comp, &stats);
            }

            local_upd_queue = local_next_upd_queue;
            local_next_upd_queue = self.to_upd.clone();
        }
    }
}
