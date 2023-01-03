use super::{
    component::{CompEvent, Component},
    prelude::ComponentCast,
    primitives::prelude::*,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Address to a pin of a specific component.
///
/// The type of pin (Input/Output) is inferred in the use of the structure. (e.g.
/// in the `ComposedComponentBuilder::connect` method where the first argument 'from'
/// represents the address of an output pin and the second one 'to' represents the
/// address of an input pin).
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct PinAddr {
    pub idx: usize,
    pub addr: usize,
}

impl PinAddr {
    /// Creates a new `PinAddr`.
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
    pub fn new(idx: usize, addr: usize) -> PinAddr {
        PinAddr { idx, addr }
    }
}

/// Macro to declare a `PinAddr` in a simple way.
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
pub struct Conn {
    pub from: PinAddr,
    pub to: PinAddr,
}

impl Conn {
    /// Creates a new 'Conn'.
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

/// A component composed by the connection of other components.
///
/// The sub-components are updated according the dependencies between them created
/// by the connections.
#[derive(Debug)]
pub struct ComposedComponent {
    pub name: String,
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,

    pub components: Vec<Box<dyn Component>>,
    pub dep_map: Vec<Vec<usize>>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<PinAddr>,
    pub out_addrs: Vec<PinAddr>,
}

impl ComponentCast for ComposedComponent {
    fn as_composed(&self) -> Option<&ComposedComponent> {
        Some(self)
    }
}

impl Component for ComposedComponent {
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

    fn on_event(&mut self, event: &CompEvent) {
        match event {
            CompEvent::UpdateValues => self.check_values(),
            CompEvent::Update(_) => {
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
            self.components[in_addr.idx].set_in(in_addr.addr, self.ins[i]);
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
        let mut visits = vec![0; self.components.len()];

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
            let sub = &mut self.components[idx];

            // Check for updates in the input values for this
            // component
            let mut j = 0;
            while j < new_inputs.len() {
                let (pin_addr, val) = &new_inputs[j];
                if pin_addr.idx == idx {
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
                sub.on_event(&CompEvent::UpdateValues);

                // Mark the current component as updated
                visits[idx] = 2;

                // Store the input values of the components that depends on the
                // recently updated one for future update of those.
                for conn in &self.connections {
                    if conn.from.idx == idx {
                        let val = sub.outs()[conn.from.addr];
                        new_inputs.push((conn.to.clone(), val));
                    }
                }

                stack.pop();
            }
        }

        // Set outputs
        for (i, out_addr) in self.out_addrs.iter().enumerate() {
            self.outs[i] = self.components[out_addr.idx].outs()[out_addr.addr];
        }
    }
}

/// Builder for the `ComposedComponent` structure.
///
/// # Example
///
/// The next example shows how to create an SR-Latch using the primitives
/// components:
///
///                  SR-Latch
///            ____________________
///           |     _______        |
///     in1 --o----|       |       |
///           |    |  NOR  |--o----o-- out1
///           | .--|_______|  |    |
///           | `-------------|-,  |
///           |  .____________' |  |
///           | |   _______     |  |
///           | `--|       |    |  |
///           |    |  NOR  |----o--o-- out2
///     in2 --o----|_______|       |
///           |____________________|
///
/// ```
/// let mut id = IDFactory::new();
/// let sr_latch = ComposedComponentBuilder::new("SRLatch")
///     .add_comp(Box::new(InputPin::new(id.set("in1"))))
///     .add_comp(Box::new(InputPin::new(id.set("in2"))))
///     .add_comp(Box::new(NorGate::new(id.set("nor1"), 2)))
///     .add_comp(Box::new(NorGate::new(id.set("nor2"), 2)))
///     .add_comp(Box::new(OutputPin::new(id.set("out1"))))
///     .add_comp(Box::new(OutputPin::new(id.set("out2"))))
///     .connect(pin!(id.get("in1"), 0), pin!(id.get("nor1"), 0))
///     .connect(pin!(id.get("in2"), 0), pin!(id.get("nor2"), 1))
///     .connect(pin!(id.get("nor1"), 0), pin!(id.get("nor2"), 0))
///     .connect(pin!(id.get("nor2"), 0), pin!(id.get("nor1"), 1))
///     .connect(pin!(id.get("nor1"), 0), pin!(id.get("out1"), 0))
///     .connect(pin!(id.get("nor2"), 0), pin!(id.get("out2"), 0))
///     .build();
/// ```
#[derive(Default)]
pub struct ComposedComponentBuilder {
    name: String,

    idx_map: HashMap<usize, Box<dyn Component>>,
    dep_map: Vec<Vec<usize>>,
    connections: Vec<Conn>,
    in_addrs: Vec<PinAddr>,
    out_addrs: Vec<PinAddr>,
}

impl ComposedComponentBuilder {
    /// Creates a new `ComposedComponentBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ComposedComponentBuilder::new();
    /// ```
    pub fn new(name: &str) -> Self {
        ComposedComponentBuilder {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Assigns the `name` for the `ComposedComponent` that will be built
    /// and returns the updated `ComposedComponentBuilder`.
    ///
    /// # Arguments
    ///
    /// * `id` - An integer that represents the id of the future component
    pub fn name(mut self, name: &str) -> ComposedComponentBuilder {
        self.name = name.to_string();
        self
    }

    /// Adds a component and returns the updated `ComposedComponentBuilder`.
    ///
    /// # Arguments
    ///
    /// * `comp` - A box containing the component to be added
    pub fn add_comp(mut self, id: usize, comp: Box<dyn Component>) -> ComposedComponentBuilder {
        self.idx_map.insert(id, comp);
        self
    }

    /// Removes a component and returns the updated `ComposedComponentBuilder`.
    ///
    /// # Arguments
    ///
    /// * `id` - An integer representing the id of the component to be removed
    pub fn remove_comp(mut self, id: usize) -> ComposedComponentBuilder {
        self.idx_map.remove(&id);
        self
    }

    /// Connects two component pins and returns the updated `ComposedComponentBuilder`.
    ///
    /// # Arguments
    ///
    /// * `from` - A `PinAddr` representing the starting point of the connection.
    /// * `to` - A `PinAddr` representing the end point of the connection.
    pub fn connect(mut self, from: PinAddr, to: PinAddr) -> ComposedComponentBuilder {
        // Check no other connection to the same input
        for conn in &self.connections {
            if conn.from == from && conn.to == to {
                // Connection already exists
                return self;
            }
            if conn.to == to {
                // Two connections to the same input pin
                panic!(
                    "Input pin {} for component {} has already a connection",
                    to.addr, to.idx
                );
            }
        }

        for (id, comp) in &self.idx_map {
            if from.idx == *id && comp.name() == Primitive::OutputPin.to_string() {
                panic!("Connecting from an output pin")
            }
            if to.idx == *id && comp.name() == Primitive::InputPin.to_string() {
                panic!("Connecting to an input pin")
            }
        }
        let conn = Conn::new(from, to);
        self.connections.push(conn);
        self
    }

    /// Disconnect two component pins and returns the updated `ComposedComponentBuilder`.
    ///
    /// # Arguments
    ///
    /// * `from` - A `PinAddr` representing the starting point of the connection.
    /// * `to` - A `PinAddr` representing the end point of the connection..
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

    /// Builds the `ComposedComponent`.
    ///
    /// Here the `idx_map` and the `dep_map` are estimated.
    ///
    /// If the component doesn't have and id and a name assiganted
    /// the build will fail.
    pub fn build(mut self) -> ComposedComponent {
        self.dep_map = Default::default();
        let mut id_to_idx: HashMap<usize, usize> = Default::default();

        for (i, (id, comp)) in self.idx_map.iter().enumerate() {
            id_to_idx.insert(*id, i);
            self.dep_map.push(vec![]);
            if comp.name() == Primitive::InputPin.to_string() {
                self.in_addrs.push(pin!(i, 0));
            }
            if comp.name() == Primitive::OutputPin.to_string() {
                self.out_addrs.push(pin!(i, 0));
            }
        }

        let mut connections: Vec<Conn> = Default::default();
        for conn in &self.connections {
            let from_idx = id_to_idx[&conn.from.idx];
            let to_idx = id_to_idx[&conn.to.idx];
            self.dep_map[to_idx].push(from_idx);
            connections.push(Conn {
                from: pin!(from_idx, conn.from.addr),
                to: pin!(to_idx, conn.to.addr),
            })
        }

        let mut components: Vec<Box<dyn Component>> = vec![];
        let ids: Vec<usize> = self.idx_map.keys().copied().collect();
        for id in ids {
            components.push(self.idx_map.remove(&id).unwrap());
        }

        ComposedComponent {
            name: self.name,
            ins: vec![false; self.in_addrs.len()],
            outs: vec![false; self.out_addrs.len()],
            dep_map: self.dep_map,
            components,
            connections,
            in_addrs: self.in_addrs,
            out_addrs: self.out_addrs,
        }
    }
}
