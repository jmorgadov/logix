use std::collections::HashMap;

use super::{
    component::{Component, SimEvent},
    primitives::primitive::Primitive,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PinAddr {
    pub id: u32,
    pub addr: usize,
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
    pub from: PinAddr,
    pub to: PinAddr,
}

impl Conn {
    pub fn new(from: PinAddr, to: PinAddr) -> Conn {
        Conn { from, to }
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

pub struct ComposedComponent {
    pub id: u32,
    pub name: String,
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,

    pub components: Vec<Box<dyn Component>>,
    pub idx_map: HashMap<u32, usize>,
    pub dep_map: Vec<Vec<usize>>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<PinAddr>,
    pub out_addrs: Vec<PinAddr>,
}

impl Component for ComposedComponent {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn is_dirty(&self) -> bool {
        self.components.iter().any(|comp| comp.is_dirty())
    }

    fn on_event(&mut self, event: &SimEvent) {
        match event {
            SimEvent::UpdateValues => self.check_values(),
            SimEvent::Update(_) => {
                self.components
                    .iter_mut()
                    .for_each(|comp| comp.on_event(event));
            }
        }
    }
}

impl ComposedComponent {
    fn check_values(&mut self) {
        // Set the inputs
        for (i, in_addr) in self.in_addrs.iter().enumerate() {
            let idx = self.idx_map[&in_addr.id];
            self.components[idx].set_in(in_addr.addr, self.ins[i]);
        }

        // Update the component
        //
        // The visits vector contains the status of all the components
        // in the updating process.
        let mut i = 0;
        let mut visits = vec![CompStatus::NotUpdated; self.components.len()];

        // This vector contains the updated values for the
        // inner connections.
        let mut new_inputs: Vec<(PinAddr, bool)> = Default::default();

        let mut stack = vec![];
        while !stack.is_empty() || i < visits.len() {
            if stack.is_empty() {
                // Check if there are unvisited components
                stack.push(i);
                i += 1;
                while i < visits.len() && visits[i].updated() {
                    i += 1
                }
            }
            let idx = stack[stack.len() - 1];
            let sub = &mut self.components[idx];

            // Check for updates in the input values for this
            // component
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

            // Check if the current component is ready to update
            // according the state of its dependencies.
            // Here the dependencie cycles can be checked if needed.
            let deps = &self.dep_map[idx];
            let mut ready_to_upd = true;
            for dep in deps {
                if visits[*dep].not_updated() {
                    ready_to_upd = false;
                    visits[*dep] = CompStatus::InUpdate;
                    stack.push(*dep);
                }
            }

            if ready_to_upd {
                visits[idx] = CompStatus::Updated;
                sub.on_event(&SimEvent::UpdateValues);

                // Store the input values of the components
                // that depends on the recently updated one
                // for future update.
                for conn in &self.connections {
                    if conn.from.id == sub.id() {
                        let val = sub.outs()[conn.from.addr];
                        new_inputs.push((conn.to.clone(), val));
                    }
                }

                stack.pop();
            }
        }

        // Set outputs
        for (i, out_addr) in self.out_addrs.iter().enumerate() {
            let idx = self.idx_map[&out_addr.id];
            self.outs[i] = self.components[idx].outs()[out_addr.addr];
        }
    }
}

#[derive(Default)]
pub struct ComposedComponentBuilder {
    id: Option<u32>,
    name: Option<String>,

    components: Vec<Box<dyn Component>>,
    idx_map: HashMap<u32, usize>,
    dep_map: Vec<Vec<usize>>,
    connections: Vec<Conn>,
    in_addrs: Vec<PinAddr>,
    out_addrs: Vec<PinAddr>,
}

impl ComposedComponentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn id(mut self, id: u32) -> ComposedComponentBuilder {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: &str) -> ComposedComponentBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn add_comp(mut self, comp: Box<dyn Component>) -> ComposedComponentBuilder {
        self.components.push(comp);
        self
    }

    pub fn remove_comp(mut self, idx: usize) -> ComposedComponentBuilder {
        self.components.remove(idx);
        self
    }

    pub fn remove_comp_by_id(mut self, comp_id: u32) -> ComposedComponentBuilder {
        for i in 0..self.components.len() {
            if self.components[i].id() == comp_id {
                self.components.remove(i);
                break;
            }
        }
        self
    }

    pub fn connect(mut self, from: PinAddr, to: PinAddr) -> ComposedComponentBuilder {
        for comp in &self.components {
            if from.id == comp.id() && comp.name() == "PinOutput" {
                panic!("Connecting from an output pin")
            }
            if to.id == comp.id() && comp.name() == "PinInput" {
                panic!("Connecting to an input pin")
            }
        }
        let conn = Conn::new(from, to);
        self.connections.push(conn);
        self
    }

    pub fn disconnect(mut self, from: PinAddr, to: PinAddr) -> ComposedComponentBuilder {
        let mut i = 0;
        while i < self.connections.len() {
            let conn = &self.connections[i];
            if conn.from == from && conn.to == to {
                self.connections.remove(i);
            }
            i += 1;
        }
        self
    }

    pub fn build(mut self) -> ComposedComponent {
        self.dep_map = Default::default();
        self.idx_map = Default::default();
        for (i, comp) in self.components.as_slice().iter().enumerate() {
            self.idx_map.insert(comp.id(), i);
            self.dep_map.push(vec![]);
            if comp.name() == Primitive::InputPin.to_string() {
                self.in_addrs.push(pin!(comp.id(), 0));
            }
            if comp.name() == Primitive::OutputPin.to_string() {
                self.out_addrs.push(pin!(comp.id(), 0));
            }
        }

        for conn in &self.connections {
            let from_idx = self.idx_map[&conn.from.id];
            let to_idx = self.idx_map[&conn.to.id];
            self.dep_map[to_idx].push(from_idx);
        }

        ComposedComponent {
            id: self
                .id
                .expect("Id must be given to build composed component"),
            name: self
                .name
                .expect("Name must be given to build composed component"),
            ins: vec![false; self.in_addrs.len()],
            outs: vec![false; self.out_addrs.len()],
            idx_map: self.idx_map,
            dep_map: self.dep_map,
            components: self.components,
            connections: self.connections,
            in_addrs: self.in_addrs,
            out_addrs: self.out_addrs,
        }
    }
}
