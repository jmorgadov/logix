use std::collections::HashMap;

pub trait Component {
    fn id(&self) -> u32;
    fn ins(&self) -> &Vec<bool>;
    fn outs(&self) -> &Vec<bool>;
    fn set_in(&mut self, idx: usize, val: bool);
    fn set_out(&mut self, idx: usize, val: bool);
    fn is_dirty(&self) -> bool;
    fn check_values(&mut self);
    fn update(&mut self, _time: u128) {}
}

pub struct BaseComponent {
    pub id: u32,
    pub upd_fn: fn(&mut BaseComponent),
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
    pub sub_comp: Option<SubComps>,
}

impl Component for BaseComponent {
    fn id(&self) -> u32 {
        self.id
    }

    fn ins(&self) -> &Vec<bool> {
        &self.ins
    }

    fn outs(&self) -> &Vec<bool> {
        &self.outs
    }

    fn is_dirty(&self) -> bool {
        match &self.sub_comp {
            Some(sub_com) => sub_com.components.iter().any(|comp| comp.is_dirty()),
            None => false,
        }
    }

    fn check_values(&mut self) {
        (self.upd_fn)(self)
    }

    fn set_in(&mut self, idx: usize, val: bool) {
        self.ins[idx] = val
    }

    fn set_out(&mut self, idx: usize, val: bool) {
        self.outs[idx] = val
    }

    fn update(&mut self, time: u128) {
        if let Some(sub_com) = &mut self.sub_comp {
            for comp in &mut sub_com.components {
                comp.update(time);
            }
        }
    }
}

impl BaseComponent {
    pub fn set_ins(&mut self, ins: Vec<bool>) -> &mut BaseComponent {
        assert!(self.ins.len() == ins.len(), "Inputs lengths don't match");
        self.ins = ins;
        self
    }
}

#[derive(Default)]
pub struct ComponentBuilder {
    id: Option<u32>,
    upd_fn: Option<fn(&mut BaseComponent)>,
    ins: Option<Vec<bool>>,
    outs: Option<Vec<bool>>,
    sub_comps: Option<SubComps>,
}

impl ComponentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn id(mut self, id: u32) -> ComponentBuilder {
        self.id = Some(id);
        self
    }

    pub fn upd_fn(mut self, upd_fn: fn(&mut BaseComponent)) -> ComponentBuilder {
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

    pub fn build(self) -> BaseComponent {
        BaseComponent {
            id: self.id.expect("Can not build component without id"),
            upd_fn: self.upd_fn.unwrap_or(|_comp| {}),
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
    id: u32,
    addr: usize,
}

impl PinAddr {
    pub fn new(id: u32, idx: usize) -> PinAddr {
        PinAddr { id, addr: idx }
    }
}

#[macro_export]
macro_rules! pin {
    ($a:expr,$b:expr) => {
        PinAddr::new($a, $b)
    };
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
    pub components: Vec<Box<dyn Component>>,
    pub idx_map: HashMap<u32, usize>,
    pub dep_map: Vec<Vec<usize>>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<PinAddr>,
    pub out_addrs: Vec<PinAddr>,
}

pub struct ComponentComposer {
    id: u32,
    comp: SubComps,
}

impl ComponentComposer {
    pub fn new(id: u32) -> Self {
        ComponentComposer {
            id,
            comp: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.comp.components.len()
    }

    pub fn add_comp(mut self, comp: Box<dyn Component>) -> ComponentComposer {
        self.comp.components.push(comp);
        self
    }

    pub fn remove_comp(mut self, idx: usize) -> ComponentComposer {
        self.comp.components.remove(idx);
        self
    }

    pub fn remove_comp_by_id(mut self, comp_id: u32) -> ComponentComposer {
        for i in 0..self.comp.components.len() {
            if self.comp.components[i].id() == comp_id {
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

    pub fn compose(mut self) -> BaseComponent {
        // Build index map and dependencies
        self.comp.dep_map = Default::default();
        self.comp.idx_map = Default::default();
        for (i, comp) in self.comp.components.as_slice().iter().enumerate() {
            self.comp.idx_map.insert(comp.id(), i);
            self.comp.dep_map.push(vec![]);
        }

        for conn in &self.comp.connections {
            let from_idx = self.comp.idx_map[&conn.from.id];
            let to_idx = self.comp.idx_map[&conn.to.id];
            self.comp.dep_map[to_idx].push(from_idx);
            // self.comp.idx_map.insert(comp.id.to_string(), i);
        }

        // Extract empty I/O pin addresses
        let mut used_inputs: Vec<Vec<bool>> = self
            .comp
            .components
            .as_slice()
            .iter()
            .map(|comp| comp.ins().to_vec())
            .collect();
        let mut used_outputs: Vec<Vec<bool>> = self
            .comp
            .components
            .as_slice()
            .iter()
            .map(|comp| comp.outs().to_vec())
            .collect();

        for conn in self.comp.connections.as_slice() {
            let from_idx = self.comp.idx_map[&conn.from.id];
            used_outputs[from_idx][conn.from.addr] = true;
            let to_idx = self.comp.idx_map[&conn.to.id];
            used_inputs[to_idx][conn.to.addr] = true;
        }

        for (i, vals) in used_inputs.iter().enumerate() {
            for (j, used) in vals.iter().enumerate() {
                if !*used {
                    self.comp
                        .in_addrs
                        .push(PinAddr::new(self.comp.components[i].id(), j));
                }
            }
        }

        for (i, vals) in used_outputs.iter().enumerate() {
            for (j, used) in vals.iter().enumerate() {
                if !*used {
                    self.comp
                        .out_addrs
                        .push(PinAddr::new(self.comp.components[i].id(), j));
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
                    let idx = sub_comp.idx_map[&in_addr.id];
                    components[idx].set_in(in_addr.addr, comp.ins[i]);
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
                        if pin_addr.id == sub.id() {
                            sub.set_in(pin_addr.addr, *val);
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
                        sub.check_values();

                        for conn in &sub_comp.connections {
                            if conn.from.id == sub.id() {
                                let val = sub.outs()[conn.from.addr];
                                new_inputs.push((conn.to.clone(), val));
                            }
                        }

                        stack.pop();
                    }
                }

                // Set outputs
                for (i, out_addr) in sub_comp.out_addrs.iter().enumerate() {
                    let idx = sub_comp.idx_map[&out_addr.id];
                    comp.outs[i] = sub_comp.components[idx].outs()[out_addr.addr];
                }
            })
            .input_count(self.comp.in_addrs.len())
            .output_count(self.comp.out_addrs.len())
            .sub_comps(self.comp)
            .build()
    }
}
