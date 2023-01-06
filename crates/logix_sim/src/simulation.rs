use crate::primitives::prelude::*;
use logix_core::prelude::*;
use std::time::Instant;

/// Simulation.
pub struct Simulation {
    comp: Component,
    running: bool,
}

impl Simulation {
    /// Creates a new simulation given the main component.
    ///
    /// # Arguments
    ///
    /// * `comp` - A box containing the main component.
    pub fn new(comp: Component) -> Self {
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
                println!("{:?}", self.comp.outputs);
            }
        }
    }
}

fn update_time(comp: &mut Component, time: u128) -> bool {
    match Primitive::from_name(&comp.name) {
        Primitive::Clock => {
            let interv =
                u128::from_ne_bytes(comp.info.as_slice().try_into().expect("Wrong clock info"));
            // println!("{:?}", interv);
            let val = (time % (interv * 2)) > interv;
            let dirty = comp.outputs[0] != val;
            comp.outputs[0] = val;
            dirty
        }
        Primitive::Unknown => {
            if let Some(sub) = comp.sub.as_mut() {
                let mut dirty = false;
                for comp in sub.components.iter_mut() {
                    dirty |= update_time(comp, time);
                }
                return dirty;
            }
            false
        }
        _ => false,
    }
}

fn update_values(comp: &mut Component) {
    match Primitive::from_name(&comp.name) {
        Primitive::NotGate => comp.outputs[0] = !comp.inputs[0],
        Primitive::AndGate => comp.outputs[0] = comp.inputs.as_slice().iter().all(|val| *val),
        Primitive::NandGate => comp.outputs[0] = !comp.inputs.as_slice().iter().all(|val| *val),
        Primitive::OrGate => comp.outputs[0] = comp.inputs.as_slice().iter().any(|val| *val),
        Primitive::NorGate => comp.outputs[0] = !comp.inputs.as_slice().iter().any(|val| *val),
        Primitive::XorGate => {
            let mut out = false;
            for i in 1..comp.inputs.len() {
                if comp.inputs[i - 1] != comp.inputs[i] {
                    out = true;
                    break;
                }
            }
            comp.inputs[0] = out;
        }
        Primitive::Unknown => {
            if let Some(sub) = comp.sub.as_mut() {
                // Set the inputs
                for (i, pin) in sub.in_addrs.iter().enumerate() {
                    sub.components[idx_of(*pin)].inputs[addr_of(*pin)] = comp.inputs[i];
                }

                // Update the component
                //
                // The visits vector contains the status of all the components
                // in the updating process.
                //  - 0 means not updated
                //  - 1 means in update process (have dependencies)
                //  - 2 means updated
                //
                let mut i = 0;
                let mut visits = vec![0; sub.components.len()];

                // This vector contains the updated values for the
                // inner connections.
                let mut new_inputs: Vec<(PortAddr, bool)> = Default::default();

                let mut stack = vec![];
                while !stack.is_empty() || i < visits.len() {
                    if stack.is_empty() {
                        // Check if there are unvisited components
                        stack.push(i);
                        i += 1;
                        while i < visits.len() && visits[i] == 2 {
                            i += 1
                        }
                    }
                    let idx = stack[stack.len() - 1];
                    let sub_comp = &mut sub.components[idx];

                    // Check for updates in the input values for this
                    // component
                    let mut j = 0;
                    while j < new_inputs.len() {
                        let (pin, val) = new_inputs[j];
                        if idx_of(pin) == idx {
                            sub_comp.inputs[addr_of(pin)] = val;
                            new_inputs.remove(j);
                            continue;
                        }
                        j += 1;
                    }

                    // Check if the current component is ready to update
                    // according the state of its dependencies.
                    // Here the dependencie cycles can be checked if needed.
                    let deps = &sub.dep_map[idx];
                    let mut ready_to_upd = true;
                    for dep in deps {
                        if visits[*dep] == 0 {
                            // If the dependency is not updated then the current component
                            // is not ready to update yet.
                            ready_to_upd = false;

                            // Then, push the dependency to the stack and mark it as
                            // in update process.
                            stack.push(*dep);
                            visits[*dep] = 1;
                        }
                    }

                    if ready_to_upd {
                        update_values(sub_comp);

                        // Mark the current component as updated
                        visits[idx] = 2;

                        // Store the input values of the components that depends on the
                        // recently updated one for future update of those.
                        for conn in &sub.connections {
                            if idx_of(conn.from) == idx {
                                let val = sub_comp.outputs[addr_of(conn.from)];
                                new_inputs.push((conn.to, val));
                            }
                        }

                        stack.pop();
                    }
                }

                // Set outputs
                for (i, pin) in sub.out_addrs.iter().enumerate() {
                    comp.outputs[i] = sub.components[idx_of(*pin)].outputs[addr_of(*pin)];
                }
            }
        }
        _ => (),
    }
}
