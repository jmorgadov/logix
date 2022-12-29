use std::collections::HashMap;

pub struct Component {
    pub id: String,
    pub upd_fn: fn(&mut Component),
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
    pub sub_comp: Option<SubComps>,
}

impl Component {
    pub fn update(&mut self) -> &mut Component {
        (self.upd_fn)(self);
        self
    }

    pub fn set_ins(&mut self, ins: Vec<bool>) -> &mut Component {
        assert!(self.ins.len() == ins.len(), "Inputs lengths don't match");
        self.ins = ins;
        self
    }
}

#[derive(Default)]
pub struct ComponentBuilder {
    id: Option<String>,
    upd_fn: Option<fn(&mut Component)>,
    ins: Option<Vec<bool>>,
    outs: Option<Vec<bool>>,
    sub_comps: Option<SubComps>,
}

impl ComponentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn id(mut self, id: String) -> ComponentBuilder {
        self.id = Some(id);
        self
    }

    pub fn upd_fn(mut self, upd_fn: fn(&mut Component)) -> ComponentBuilder {
        self.upd_fn = Some(upd_fn);
        self
    }

    pub fn inputs(mut self, inputs: Vec<bool>) -> ComponentBuilder {
        self.ins = Some(inputs);
        self
    }

    pub fn outputs(mut self, outputs: Vec<bool>) -> ComponentBuilder {
        self.outs = Some(outputs);
        self
    }

    pub fn input_count(mut self, count: usize) -> ComponentBuilder {
        self.ins = Some((0..count).map(|_| false).collect());
        self
    }

    pub fn output_count(mut self, count: usize) -> ComponentBuilder {
        self.outs = Some((0..count).map(|_| false).collect());
        self
    }

    pub fn sub_comps(mut self, sub_comps: SubComps) -> ComponentBuilder {
        self.sub_comps = Some(sub_comps);
        self
    }

    pub fn build(self) -> Component {
        Component {
            id: self.id.expect("Can not build component without id"),
            upd_fn: self
                .upd_fn
                .expect("Can not build component without update function"),
            ins: self.ins.unwrap_or_else(|| vec![false]),
            outs: self.outs.unwrap_or_else(|| vec![false]),
            sub_comp: self.sub_comps,
        }
    }
}

#[derive(Clone)]
enum CompStatus {
    NotUpdated,
    InUpdate,
    Updated,
}

impl CompStatus {
    pub fn not_updated(&self) -> bool {
        matches!(self, CompStatus::NotUpdated)
    }

    pub fn in_update(&self) -> bool {
        matches!(self, CompStatus::InUpdate)
    }

    pub fn updated(&self) -> bool {
        matches!(self, CompStatus::Updated)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct PinAddr {
    id: String,
    addr: usize,
}

impl PinAddr {
    pub fn new(id: &str, idx: usize) -> PinAddr {
        PinAddr {
            id: id.to_string(),
            addr: idx,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Conn {
    from: PinAddr,
    to: PinAddr,
}

impl Conn {
    pub fn new(from: PinAddr, to: PinAddr) -> Conn {
        Conn { from, to }
    }
}

#[derive(Default)]
pub struct SubComps {
    pub components: Vec<Component>,
    pub idx_map: HashMap<String, usize>,
    pub dep_map: Vec<Vec<usize>>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<PinAddr>,
    pub out_addrs: Vec<PinAddr>,
}

pub struct ComponentComposer {
    id: String,
    comp: SubComps,
}

impl ComponentComposer {
    pub fn new(id: &str) -> Self {
        ComponentComposer {
            id: id.to_string(),
            comp: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.comp.components.len()
    }

    pub fn add_comp(mut self, comp: Component) -> ComponentComposer {
        self.comp.components.push(comp);
        self
    }

    pub fn remove_comp(mut self, idx: usize) -> ComponentComposer {
        self.comp.components.remove(idx);
        self
    }

    pub fn remove_comp_by_id(mut self, comp_id: &String) -> ComponentComposer {
        for i in 0..self.comp.components.len() {
            if self.comp.components[i].id == *comp_id {
                self.comp.components.remove(i);
                break;
            }
        }
        self
    }

    pub fn connect(mut self, from: PinAddr, to: PinAddr) -> ComponentComposer {
        let conn = Conn::new(from, to);
        self.comp.connections.push(conn);
        self
    }

    pub fn disconnect(mut self, from: PinAddr, to: PinAddr) -> ComponentComposer {
        let mut i = 0;
        while i < self.comp.connections.len() {
            let conn = &self.comp.connections[i];
            if conn.from == from && conn.to == to {
                self.comp.connections.remove(i);
            }
            i += 1;
        }
        self
    }

    pub fn compose(mut self) -> Component {
        // Build index map and dependencies
        self.comp.dep_map = Default::default();
        self.comp.idx_map = Default::default();
        for (i, comp) in self.comp.components.as_slice().iter().enumerate() {
            self.comp.idx_map.insert(comp.id.to_string(), i);
            self.comp.dep_map.push(vec![]);
        }

        for conn in &self.comp.connections {
            let from_idx = self.comp.idx_map[&conn.from.id.to_string()];
            let to_idx = self.comp.idx_map[&conn.to.id.to_string()];
            self.comp.dep_map[to_idx].push(from_idx);
            // self.comp.idx_map.insert(comp.id.to_string(), i);
        }

        // Extract empty I/O pin addresses
        let mut used_inputs: Vec<Vec<bool>> = self
            .comp
            .components
            .as_slice()
            .iter()
            .map(|comp| comp.ins.to_vec())
            .collect();
        let mut used_outputs: Vec<Vec<bool>> = self
            .comp
            .components
            .as_slice()
            .iter()
            .map(|comp| comp.outs.to_vec())
            .collect();

        for conn in self.comp.connections.as_slice() {
            let from_idx = self.comp.idx_map[&conn.from.id.to_string()];
            used_outputs[from_idx][conn.from.addr] = true;
            let to_idx = self.comp.idx_map[&conn.to.id.to_string()];
            used_inputs[to_idx][conn.to.addr] = true;
        }

        for (i, vals) in used_inputs.iter().enumerate() {
            for (j, used) in vals.iter().enumerate() {
                if !*used {
                    self.comp
                        .in_addrs
                        .push(PinAddr::new(&self.comp.components[i].id, j));
                }
            }
        }

        for (i, vals) in used_outputs.iter().enumerate() {
            for (j, used) in vals.iter().enumerate() {
                if !*used {
                    self.comp
                        .out_addrs
                        .push(PinAddr::new(&self.comp.components[i].id, j));
                }
            }
        }

        ComponentBuilder::new()
            .id(self.id)
            .upd_fn(|comp| {
                let sub_comp = comp.sub_comp.as_mut().unwrap();
                let components = &mut sub_comp.components;

                // Set the inputs
                for (i, in_addr) in sub_comp.in_addrs.iter().enumerate() {
                    let idx = sub_comp.idx_map[&in_addr.id.to_string()];
                    components[idx].ins[in_addr.addr] = comp.ins[i];
                }

                // Update
                let mut i = 0;
                let mut visits = vec![];
                visits.resize(components.len(), CompStatus::NotUpdated);

                let mut new_inputs: Vec<(PinAddr, bool)> = Default::default();
                let mut stack = vec![];
                while !stack.is_empty() || i < visits.len() {
                    if stack.is_empty() {
                        stack.push(i);
                        i += 1;
                        while i < visits.len() && visits[i].updated() {
                            i += 1
                        }
                    }
                    let idx = stack[stack.len() - 1];
                    let sub = &mut components[idx];

                    // Check new input values for this sub component
                    let mut j = 0;
                    while j < new_inputs.len() {
                        let (pin_addr, val) = &new_inputs[j];
                        if pin_addr.id == sub.id {
                            sub.ins[pin_addr.addr] = *val;
                            new_inputs.remove(j);
                            continue;
                        }
                        j += 1;
                    }

                    let deps = &sub_comp.dep_map[idx];
                    let mut ready_to_upd = true;
                    for dep in deps {
                        if visits[*dep].in_update() {
                            panic!("Cycle encountered")
                        }
                        if visits[*dep].not_updated() {
                            ready_to_upd = false;
                            visits[*dep] = CompStatus::InUpdate;
                            stack.push(*dep);
                        }
                    }

                    if ready_to_upd {
                        visits[idx] = CompStatus::Updated;
                        sub.update();

                        for conn in &sub_comp.connections {
                            if conn.from.id == sub.id {
                                let val = sub.outs[conn.from.addr];
                                new_inputs.push((conn.to.clone(), val));
                            }
                        }

                        stack.pop();
                    }
                }

                // Set outputs
                for (i, out_addr) in sub_comp.out_addrs.iter().enumerate() {
                    let idx = sub_comp.idx_map[&out_addr.id.to_string()];
                    comp.outs[i] = sub_comp.components[idx].outs[out_addr.addr];
                }
            })
            .input_count(self.comp.in_addrs.len())
            .output_count(self.comp.out_addrs.len())
            .sub_comps(self.comp)
            .build()
    }
}
