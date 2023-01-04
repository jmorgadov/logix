use super::{
    component::{CompEvent, Component},
    prelude::ComponentCast,
};

/// Address to a pin of a specific component.
///
/// The first item of the tuple represents the component index
/// and the second one represents the pin addr (index of and input/output).
///
/// The type of pin (Input/Output) is inferred in the use of the structure.
pub type PinAddr = (usize, usize);

#[inline(always)]
fn idx_of(pin_addr: PinAddr) -> usize {
    pin_addr.0
}

#[inline(always)]
fn addr_of(pin_addr: PinAddr) -> usize {
    pin_addr.1
}

/// Represents a connection between two component pins.
///
/// The address stored in `from` is assumed to be from an output pin
/// and the one stored in `to` is assumed to be to an input pin.
#[derive(PartialEq, Eq, Debug)]
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
    /// use logix::prelude::Conn;
    /// let conn = Conn::new((10, 0), (20, 3));
    /// ```
    pub fn new(from: PinAddr, to: PinAddr) -> Conn {
        Conn { from, to }
    }
}

#[macro_export]
macro_rules! conn {
    ($a:expr,$b:expr) => {
        Conn::new($a, $b)
    };
}

#[derive(Default, Debug)]
pub struct ComponentBuildError;

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
    fn build(
        name: &str,
        components: Vec<Box<dyn Component>>,
        connections: Vec<Conn>,
        in_addrs: Vec<PinAddr>,
        out_addrs: Vec<PinAddr>,
    ) -> Result<Self, ComponentBuildError> {
        let mut dep_map = vec![vec![]; components.len()];
        for conn in &connections {
            dep_map[idx_of(conn.to)].push(idx_of(conn.from));
        }

        Ok(ComposedComponent {
            name: name.to_string(),
            ins: vec![false; in_addrs.len()],
            outs: vec![false; out_addrs.len()],
            components,
            dep_map,
            connections,
            in_addrs,
            out_addrs,
        })
    }
    fn check_values(&mut self) {
        // Set the inputs
        for (i, pin) in self.in_addrs.iter().enumerate() {
            self.components[idx_of(*pin)].set_in(addr_of(*pin), self.ins[i]);
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
                    if idx_of(conn.from) == idx {
                        let val = sub.outs()[addr_of(conn.from)];
                        new_inputs.push((conn.to, val));
                    }
                }

                stack.pop();
            }
        }

        // Set outputs
        for (i, pin) in self.out_addrs.iter().enumerate() {
            self.outs[i] = self.components[idx_of(*pin)].outs()[addr_of(*pin)];
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
/// ```
/// use logix::prelude::*;
/// let sr_latch = ComposedComponentBuilder::new("SRLatch")
///     .components(vec![Box::new(NorGate::new(2)), Box::new(NorGate::new(2))])
///     .connections(vec![conn!((0, 0), (1, 0)), conn!((1, 0), (0, 1))])
///     .inputs(vec![(0, 0), (1, 1)])
///     .outputs(vec![(0, 0), (1, 0)])
///     .build()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct ComposedComponentBuilder {
    name: String,

    components: Vec<Box<dyn Component>>,
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
    /// use logix::prelude::ComposedComponentBuilder;
    /// let builder = ComposedComponentBuilder::new("MyComp");
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
    /// * `name` - A string slice that holds the new component name.
    pub fn name(mut self, name: &str) -> ComposedComponentBuilder {
        self.name = name.to_string();
        self
    }

    /// Adds a component and returns the updated `ComposedComponentBuilder`.
    ///
    /// # Arguments
    ///
    /// * `components` - A vector of boxed components.
    pub fn components(mut self, components: Vec<Box<dyn Component>>) -> ComposedComponentBuilder {
        self.components = components;
        self
    }

    /// Connects two component pins and returns the updated `ComposedComponentBuilder`.
    ///
    /// # Arguments
    ///
    /// * `from` - A `PinAddr` representing the starting point of the connection.
    /// * `to` - A `PinAddr` representing the end point of the connection.
    pub fn connections(mut self, connections: Vec<Conn>) -> ComposedComponentBuilder {
        self.connections = connections;
        self
    }

    pub fn inputs(mut self, inputs: Vec<PinAddr>) -> ComposedComponentBuilder {
        self.in_addrs = inputs;
        self
    }

    pub fn outputs(mut self, outputs: Vec<PinAddr>) -> ComposedComponentBuilder {
        self.out_addrs = outputs;
        self
    }

    /// Builds the `ComposedComponent`.
    pub fn build(self) -> Result<ComposedComponent, ComponentBuildError> {
        ComposedComponent::build(
            &self.name,
            self.components,
            self.connections,
            self.in_addrs,
            self.out_addrs,
        )
    }
}
