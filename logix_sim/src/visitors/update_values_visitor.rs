use logix::prelude::*;

pub struct UpdateValuesVisitor;

impl UpdateValuesVisitor {
    fn visit_not_gate(comp: &mut NotGate) {
        comp.outs[0] = !comp.ins[0];
    }

    fn visit_and_gate(comp: &mut AndGate) {
        let out: bool = comp.ins.as_slice().iter().all(|val| *val);
        comp.outs[0] = out;
    }

    fn visit_or_gate(comp: &mut OrGate) {
        let out: bool = comp.ins.as_slice().iter().any(|val| *val);
        comp.outs[0] = out;
    }

    fn visit_nand_gate(comp: &mut NandGate) {
        let out: bool = comp.ins.as_slice().iter().all(|val| *val);
        comp.outs[0] = !out;
    }

    fn visit_nor_gate(comp: &mut NorGate) {
        let out: bool = comp.ins.as_slice().iter().any(|val| *val);
        comp.outs[0] = !out;
    }

    fn visit_xor_gate(comp: &mut XorGate) {
        let mut out = false;
        for i in 1..comp.ins.len() {
            if comp.ins[i - 1] != comp.ins[i] {
                out = true;
                break;
            }
        }
        comp.outs[0] = out;
    }

    pub fn visit_composed(comp: &mut ComposedComponent) {
        // Set the inputs
        for (i, pin) in comp.in_addrs.iter().enumerate() {
            comp.components[idx_of(*pin)].set_in(addr_of(*pin), comp.ins[i]);
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
        let mut visits = vec![0; comp.components.len()];

        // This vector contains the updated values for the
        // inner connections.
        let mut new_inputs: Vec<(PinAddr, bool)> = Default::default();

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
            let sub = &mut comp.components[idx];

            // Check for updates in the input values for this
            // component
            let mut j = 0;
            while j < new_inputs.len() {
                let (pin, val) = new_inputs[j];
                if idx_of(pin) == idx {
                    sub.set_in(addr_of(pin), val);
                    new_inputs.remove(j);
                    continue;
                }
                j += 1;
            }

            // Check if the current component is ready to update
            // according the state of its dependencies.
            // Here the dependencie cycles can be checked if needed.
            let deps = &comp.dep_map[idx];
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
                if let Ok(prim) = Primitive::from_name(&sub.name()) {
                    match prim {
                        Primitive::NotGate => {
                            UpdateValuesVisitor::visit_not_gate(sub.as_not_gate_mut().unwrap())
                        }
                        Primitive::AndGate => {
                            UpdateValuesVisitor::visit_and_gate(sub.as_and_gate_mut().unwrap())
                        }
                        Primitive::OrGate => {
                            UpdateValuesVisitor::visit_or_gate(sub.as_or_gate_mut().unwrap())
                        }
                        Primitive::NandGate => {
                            UpdateValuesVisitor::visit_nand_gate(sub.as_nand_gate_mut().unwrap())
                        }
                        Primitive::NorGate => {
                            UpdateValuesVisitor::visit_nor_gate(sub.as_nor_gate_mut().unwrap())
                        }
                        Primitive::XorGate => {
                            UpdateValuesVisitor::visit_xor_gate(sub.as_xor_gate_mut().unwrap())
                        }
                        _ => (),
                    }
                } else {
                    UpdateValuesVisitor::visit_composed(sub.as_composed_mut().unwrap())
                }

                // Mark the current component as updated
                visits[idx] = 2;

                // Store the input values of the components that depends on the
                // recently updated one for future update of those.
                for conn in &comp.connections {
                    if idx_of(conn.from) == idx {
                        let val = sub.outs()[addr_of(conn.from)];
                        new_inputs.push((conn.to, val));
                    }
                }

                stack.pop();
            }
        }

        // Set outputs
        for (i, pin) in comp.out_addrs.iter().enumerate() {
            comp.outs[i] = comp.components[idx_of(*pin)].outs()[addr_of(*pin)];
        }
    }
}
