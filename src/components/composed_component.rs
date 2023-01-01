use super::{
    component::{Component, SimEvent},
    primitives::prelude::*,
};
use crate::serialize::JSONSerialize;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Address to a pin of a specific component
///
/// The type of pin (Input/Output) is inferred in the use of the structure. (e.g.
/// in the `ComposedComponentBuilder::connect` method where the first argument 'from'
/// represents the address of an output pin and the second one 'to' represents the
/// address of an input pin)
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct PinAddr {
    pub id: u32,
    pub addr: usize,
}

impl PinAddr {
    /// Creates a new `PinAddr`
    ///
    /// # Arguments
    ///
    /// * `id` - The component id.
    /// * `idx` - A usize index that represents the address of the pin.
    ///
    /// # Examples
    ///
    /// ```
    /// let pin_addr = PinAddr::new(0, 2);
    /// ```
    ///
    /// The `pin!` macro can also be used to create a `PinAddr`
    ///
    /// ```
    /// let pin_addr = pin!(0, 2)
    /// ```
    pub fn new(id: u32, idx: usize) -> PinAddr {
        PinAddr { id, addr: idx }
    }
}

/// Macro to declare a `PinAddr` in a simple way
#[macro_export]
macro_rules! pin {
    ($a:expr,$b:expr) => {
        PinAddr::new($a, $b)
    };
}

/// Represents a connection between two component pins.
///
/// The address stored in `from` is assumed to be from an output pin
/// and the one stored in `to` is assumed to be to an input pin.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct Conn {
    pub from: PinAddr,
    pub to: PinAddr,
}

impl Conn {
    /// Creates a new 'Conn'
    ///
    /// # Arguments
    ///
    /// * `from` - A `PinAddr` representing the starting point of the connection.
    /// * `to` - A `PinAddr` representing the end point of the connection.
    ///
    /// # Examples
    ///
    /// A connection from the first output pin of the component
    /// with id `10`, to the third input pin of the component with
    /// id `20`.
    ///
    /// ```
    /// let conn = Conn::new(pin!(10, 0), pin!(20, 3));
    /// ```
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

/// A component composed by the connection of other components
///
/// The sub-components are updated according the dependencies between them created
/// by the connections.
#[derive(Debug)]
pub struct ComposedComponent {
    id: u32,
    name: String,
    ins: Vec<bool>,
    outs: Vec<bool>,

    components: Vec<Box<dyn Component>>,
    idx_map: HashMap<u32, usize>,
    dep_map: Vec<Vec<usize>>,
    connections: Vec<Conn>,
    in_addrs: Vec<PinAddr>,
    out_addrs: Vec<PinAddr>,
}

impl JSONSerialize for ComposedComponent {
    fn to_json(&self) -> Value {
        let mut val: Value = Default::default();
        let comps: Vec<Value> = self.components.iter().map(|e| e.to_json()).collect();

        val["id"] = json!(self.id);
        val["name"] = json!(self.name);
        val["connections"] = json!(self.connections);
        val["components"] = json!(comps);
        val
    }

    fn from_json(json: &Value) -> Self
    where
        Self: Sized,
    {
        let mut builder = ComposedComponentBuilder::new()
            .id(json["id"].as_u64().unwrap() as u32)
            .name(json["name"].as_str().unwrap());

        for comp_json in json["components"].as_array().unwrap().iter() {
            let name = comp_json["name"].as_str().unwrap();
            let sub_comp: Box<dyn Component>;
            if let Ok(prim) = Primitive::from_str(name) {
                match prim {
                    Primitive::NotGate => {
                        sub_comp = Box::new(NotGate::from_json(comp_json));
                    }
                    Primitive::AndGate => {
                        sub_comp = Box::new(AndGate::from_json(comp_json));
                    }
                    Primitive::OrGate => {
                        sub_comp = Box::new(OrGate::from_json(comp_json));
                    }
                    Primitive::NandGate => {
                        sub_comp = Box::new(NandGate::from_json(comp_json));
                    }
                    Primitive::NorGate => {
                        sub_comp = Box::new(NorGate::from_json(comp_json));
                    }
                    Primitive::XorGate => {
                        sub_comp = Box::new(XorGate::from_json(comp_json));
                    }
                    Primitive::Clock => {
                        sub_comp = Box::new(Clock::from_json(comp_json));
                    }
                    Primitive::InputPin => {
                        sub_comp = Box::new(InputPin::from_json(comp_json));
                    }
                    Primitive::OutputPin => {
                        sub_comp = Box::new(OutputPin::from_json(comp_json));
                    }
                    Primitive::ConstOne => {
                        sub_comp = Box::new(Const::from_json(comp_json));
                    }
                    Primitive::ConstZero => {
                        sub_comp = Box::new(Const::from_json(comp_json));
                    }
                }
            } else {
                sub_comp = Box::new(ComposedComponent::from_json(comp_json))
            }
            builder = builder.add_comp(sub_comp);
        }

        for conn_json in json["connections"].as_array().unwrap().iter() {
            let from = conn_json["from"].as_object().unwrap();
            let from_pin = pin!(
                from["id"].as_u64().unwrap() as u32,
                from["addr"].as_u64().unwrap() as usize
            );
            let to = conn_json["to"].as_object().unwrap();
            let to_pin = pin!(
                to["id"].as_u64().unwrap() as u32,
                to["addr"].as_u64().unwrap() as usize
            );
            builder = builder.connect(from_pin, to_pin);
        }
        builder.build()
    }
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

/// Builder for the `ComposedComponent` structure.
///
/// # Example
///
/// ```
/// let mut id = IDFactory::new();
/// let sr_latch = ComposedComponentBuilder::new()
///     .id(id.set("sr_latch"))
///     .name("SRLatch")
///     .add_comp(Box::new(InputPin::new(id.set("i1"))))
///     .add_comp(Box::new(InputPin::new(id.set("i2"))))
///     .add_comp(Box::new(NorGate::new(id.set("nor1"), 2)))
///     .add_comp(Box::new(NorGate::new(id.set("nor2"), 2)))
///     .add_comp(Box::new(OutputPin::new(id.set("o1"))))
///     .add_comp(Box::new(OutputPin::new(id.set("o2"))))
///     .connect(pin!(id.get("i1"), 0), pin!(id.get("nor1"), 0))
///     .connect(pin!(id.get("i2"), 0), pin!(id.get("nor2"), 1))
///     .connect(pin!(id.get("nor1"), 0), pin!(id.get("nor2"), 0))
///     .connect(pin!(id.get("nor2"), 0), pin!(id.get("nor1"), 1))
///     .connect(pin!(id.get("nor1"), 0), pin!(id.get("o1"), 0))
///     .connect(pin!(id.get("nor2"), 0), pin!(id.get("o2"), 0))
///     .build();
/// ```
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
    /// Creates a new `ComposedComponentBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ComposedComponentBuilder::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Assigns the `id` for the `ComposedComponent` that will be built
    /// and returns the updated `ComposedComponentBuilder`
    ///
    /// # Arguments
    ///
    /// * `id` - An integer that represents the id of the future component
    pub fn id(mut self, id: u32) -> ComposedComponentBuilder {
        self.id = Some(id);
        self
    }

    /// Assigns the `name` for the `ComposedComponent` that will be built
    /// and returns the updated `ComposedComponentBuilder`
    ///
    /// # Arguments
    ///
    /// * `id` - An integer that represents the id of the future component
    pub fn name(mut self, name: &str) -> ComposedComponentBuilder {
        self.name = Some(name.to_string());
        self
    }

    /// Adds a component and returns the updated `ComposedComponentBuilder`
    ///
    /// # Arguments
    ///
    /// * `comp` - A box containing the component to be added
    pub fn add_comp(mut self, comp: Box<dyn Component>) -> ComposedComponentBuilder {
        self.components.push(comp);
        self
    }

    /// Removes a component and returns the updated `ComposedComponentBuilder`
    ///
    /// # Arguments
    ///
    /// * `id` - An integer representing the id of the component to be removed
    pub fn remove_comp(mut self, id: u32) -> ComposedComponentBuilder {
        for i in 0..self.components.len() {
            if self.components[i].id() == id {
                self.components.remove(i);
                break;
            }
        }
        self
    }

    /// Connects two component pins and returns the updated `ComposedComponentBuilder`
    ///
    /// # Arguments
    ///
    /// * `from` - A `PinAddr` representing the starting point of the connection
    /// * `to` - A `PinAddr` representing the end point of the connection.
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

    /// Disconnect two component pins and returns the updated `ComposedComponentBuilder`
    ///
    /// # Arguments
    ///
    /// * `from` - A `PinAddr` representing the starting point of the connection
    /// * `to` - A `PinAddr` representing the end point of the connection.
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

    /// Builds the `ComposedComponent`
    ///
    /// Here the `idx_map` and the `dep_map` are estimated.
    ///
    /// If the component doesn't have and id and a name assiganted
    /// the build will fail.
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

// #[cfg(test)]
// mod tests {
//     use serde_json::json;

//     use crate::components::primitives::{
//         and_gate::AndGate, constant::Const, input_pin::InputPin, or_gate::OrGate,
//         output_pin::OutputPin,
//     };

//     use super::{ComposedComponentBuilder, PinAddr};

//     #[test]
//     fn to_json() {
//         let comp = ComposedComponentBuilder::new()
//             .id(0)
//             .name("TestComponent")
//             .add_comp(Box::new(InputPin::new(1)))
//             .add_comp(Box::new(InputPin::new(2)))
//             .add_comp(Box::new(Const::one(3)))
//             .add_comp(Box::new(OutputPin::new(4)))
//             .add_comp(Box::new(AndGate::new(5, 2)))
//             .add_comp(Box::new(OrGate::new(6, 2)))
//             .connect(pin!(1, 0), pin!(5, 0))
//             .connect(pin!(3, 0), pin!(5, 1))
//             .connect(pin!(5, 0), pin!(6, 0))
//             .connect(pin!(2, 0), pin!(6, 1))
//             .connect(pin!(6, 0), pin!(4, 0))
//             .build();
//         json!(comp);
//     }
// }
