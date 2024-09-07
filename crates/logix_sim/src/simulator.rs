use crate::{
    flatten::FlattenComponent,
    primitives::{prim_program::ProgramUpdateType, primitive::Primitive},
};
use log::debug;
use rand::seq::SliceRandom;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

pub struct SimState {
    pub comp: FlattenComponent,
    to_upd: Vec<usize>,
    pub running: bool,
    pub end: bool,
}

pub struct Simulator {
    state: Arc<Mutex<SimState>>,
}

impl Simulator {
    pub fn new(comp: FlattenComponent) -> Self {
        let to_upd = comp
            .components
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                match &c.prim_type {
                    Primitive::Const { value: _v } => {
                        return Some(i);
                    }
                    Primitive::Clock { period: _p } => {
                        return Some(i);
                    }
                    Primitive::Custom { prog } => {
                        if let ProgramUpdateType::Always = prog.update_type {
                            return Some(i);
                        }
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
            state: Arc::new(Mutex::new(SimState {
                comp,
                to_upd,
                running: false,
                end: false,
            })),
        }
    }

    pub fn component<T>(&mut self, on_locked: impl FnOnce(&mut FlattenComponent) -> T) -> T {
        let mut state = self.state.lock().unwrap();
        on_locked(&mut state.comp)
    }

    pub fn try_component<T: Default>(
        &mut self,
        on_locked: impl FnOnce(&mut FlattenComponent) -> T,
    ) -> T {
        let state = self.state.try_lock();
        if let Ok(mut state) = state {
            return on_locked(&mut state.comp);
        }
        T::default()
    }

    pub fn pause_resume(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.running = !state.running;
    }

    pub fn stop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.end = true;
    }

    /// Starts the simulation.
    pub fn start(&mut self, keep_running: bool) {
        let state_arc = self.state.clone();

        thread::spawn(move || {
            let mut state = state_arc.lock().unwrap();
            state.running = true;
            let start = Instant::now();
            let mut local_upd_list: Vec<usize> = (0..state.comp.components.len()).collect();
            let mut local_next_upd_list: Vec<usize> = state.to_upd.clone();
            let mut to_upd_len = state.to_upd.len();

            {
                let _x = state;
            }

            while to_upd_len > 0 || keep_running {
                // debug!("Update list: {:?}", self.to_upd);

                let mut state = state_arc.lock().unwrap();
                if state.end {
                    {
                        let _x = state;
                    }
                    break;
                }

                if !state.running {
                    {
                        let _x = state;
                    }
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                let time = start.elapsed().as_nanos();

                for idx in local_upd_list.iter() {
                    debug!(
                        "To Update: {:?}",
                        local_upd_list
                            .iter()
                            .map(|x| format!("{} {}", *x, state.comp.components[*x].name.clone()))
                            .collect::<Vec<String>>()
                    );

                    let comp_idx = *idx;
                    let comp_i = &mut state.comp.components[comp_idx];
                    debug!("Updating component: {} {:?}", comp_idx, comp_i.prim_type);
                    debug!("  Old inputs: {:?}", comp_i.inputs);
                    debug!("  Old outputs: {:?}", comp_i.outputs);

                    comp_i.update(time);

                    debug!("  New outputs: {:?}", comp_i.outputs);

                    match &comp_i.prim_type {
                        Primitive::Clock { period: _p } => {}
                        Primitive::Custom { prog } => {
                            if let ProgramUpdateType::InputChanges = prog.update_type {
                                state.to_upd.retain(|&x| x != comp_idx);
                            }
                        }
                        _ => {
                            state.to_upd.retain(|&x| x != comp_idx);
                        }
                    }

                    let mut rand_conns =
                        (0..state.comp.connections[comp_idx].len()).collect::<Vec<usize>>();

                    rand_conns.shuffle(&mut rand::thread_rng());

                    for idx in rand_conns.iter() {
                        let conn = state.comp.connections[comp_idx][*idx];
                        let val = state.comp.components[comp_idx].outputs[conn.from.1];

                        // Do not update if the value is the same
                        if val == state.comp.components[conn.to.0].inputs[conn.to.1] {
                            continue;
                        }

                        debug!("  Connection: {:?}", conn);
                        debug!(
                            "New comp to update: {} {:?}",
                            conn.to.0, state.comp.components[conn.to.0].prim_type
                        );

                        // Update the value
                        state.comp.components[conn.to.0].inputs[conn.to.1] = val;

                        if !local_next_upd_list.contains(&conn.to.0) {
                            // stats.clk_cycle_ended = false;
                            local_next_upd_list.push(conn.to.0);
                        }
                    }
                }

                local_upd_list = local_next_upd_list;
                local_next_upd_list = state.to_upd.clone();

                to_upd_len = state.to_upd.len();
                {
                    let _x = state;
                }
            }
        });
    }
}
