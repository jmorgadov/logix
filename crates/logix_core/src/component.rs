use serde::{Deserialize, Serialize};
use std::{default::Default, usize};

/// Represents a location of a port inside a Component.
///
/// The first item is the index of the component in the sub-components vector.
/// The second item is the index of the port.
///
/// The distinction between Input/Output ports depends on the context.
pub type PortAddr = (usize, usize);

/// Returns the sub-component index of a [`PortAddr`].
#[inline(always)]
pub fn idx_of(port_addr: PortAddr) -> usize {
    port_addr.0
}

/// Returns the port index of a [`PortAddr`].
#[inline(always)]
pub fn addr_of(port_addr: PortAddr) -> usize {
    port_addr.1
}

/// Represents a connection between two ports.
///
/// The port index of the `from` part is taken from the outputs of the
/// component it represents. The port index of the `to` part is taken from
/// the inputs of the component it represents.
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conn {
    pub from: PortAddr,
    pub to: PortAddr,
}

impl Conn {
    /// Creates a connection given de indexes of the components and ports of the
    /// starting and ending points of the connection.
    ///
    /// # Arguments
    ///
    /// * `from_idx` - An integer representing the index of the sub-component
    /// where the connection starts.
    /// * `from_port` - An integer representing the index of the output port
    /// where the connection starts.
    /// * `to_idx` - An integer representing the index of the sub-component
    /// where the connection ends.
    /// * `to_port` - An integer representing the index of the input port
    /// where the connection starts.
    pub fn new(from_idx: usize, from_port: usize, to_idx: usize, to_port: usize) -> Conn {
        Conn {
            from: (from_idx, from_port),
            to: (to_idx, to_port),
        }
    }
}

/// Holds all the information of the sub-components of a component.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SubComponent<E: Default + Clone + Serialize> {
    /// Vector of sub-components.
    pub components: Vec<Component<E>>,

    /// Vector that holds the connections between the sub-components.
    pub connections: Vec<Conn>,

    /// Vector that maps the component inputs to input ports of the sub-components
    pub in_addrs: Vec<(usize, PortAddr)>,

    /// Vector that maps the component outputs to outputs ports of the sub-components
    pub out_addrs: Vec<PortAddr>,
}

/// Represents a component.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Component<E: Default + Clone + Serialize> {
    /// Unique identifier of the component.
    pub id: usize,

    /// Name of the component.
    pub name: Option<String>,

    /// Input ports of the component.
    pub inputs: usize,

    /// Output ports of the component.
    pub outputs: usize,

    /// Option that holds the sub-component information.
    ///
    /// If None, then the component is consider a base component.
    pub sub: Option<SubComponent<E>>,

    // Extra information
    pub extra: E,
}

#[derive(Default)]
pub struct ComponentBuilder<E: Default + Clone + Serialize> {
    id: usize,
    name: Option<String>,
    inputs: usize,
    outputs: usize,

    sub_comps: Option<Vec<Component<E>>>,
    connections: Option<Vec<Conn>>,
    in_addrs: Option<Vec<(usize, PortAddr)>>,
    out_addrs: Option<Vec<PortAddr>>,
    extra: E,
}

impl<E: Default + Clone + Serialize> ComponentBuilder<E> {
    /// Creates a new [`ComponentBuilder`]
    pub fn new(id: usize) -> Self {
        ComponentBuilder {
            id,
            name: None,
            inputs: 0,
            outputs: 0,
            sub_comps: None,
            connections: None,
            in_addrs: None,
            out_addrs: None,
            extra: Default::default(),
        }
    }

    /// Sets the name of the component.
    ///
    /// # Arguments
    ///
    /// * `name`: String that represents the name of the component.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the amount of input ports.
    ///
    /// # Arguments
    ///
    /// * `n`: Integer that represents the amount of input ports.
    pub fn in_count(mut self, n: usize) -> Self {
        self.inputs = n;
        self
    }

    /// Sets the amount of output ports.
    ///
    /// # Arguments
    ///
    /// * `n`: Integer that represents the amount of output ports.
    pub fn out_count(mut self, n: usize) -> Self {
        self.outputs = n;
        self
    }

    /// Sets the amount of input and output ports.
    ///
    /// # Arguments
    ///
    /// * `in_count`: Integer that represents the amount of input ports.
    /// * `out_count`: Integer that represents the amount of output ports.
    pub fn port_count(mut self, in_count: usize, out_count: usize) -> Self {
        self.inputs = in_count;
        self.outputs = out_count;
        self
    }

    /// Sets the subcomponents.
    ///
    /// # Arguments
    ///
    /// * `sub_comps`: Vector of [`Components`] that holds all the sub-components.
    pub fn sub_comps(mut self, sub_comps: Vec<Component<E>>) -> Self {
        self.sub_comps = Some(sub_comps);
        self
    }

    /// Sets the connections between the subcomponents.
    ///
    /// # Arguments
    ///
    /// * `connections`: Vector of [`Conn`] that holds all the connections between
    /// the sub-components.
    pub fn connections(mut self, connections: Vec<Conn>) -> Self {
        self.connections = Some(connections);
        self
    }

    /// Sets the input port addresses.
    ///
    /// # Arguments
    ///
    /// * `in_addrs`: Vector of [`PortAddr`] that holds all input port addresses.
    pub fn in_addrs(mut self, in_addrs: Vec<(usize, PortAddr)>) -> Self {
        self.in_addrs = Some(in_addrs);
        self
    }

    /// Sets the output port addresses.
    ///
    /// # Arguments
    ///
    /// * `out_addrs`: Vector of [`PortAddr`] that holds all output port addresses.
    pub fn out_addrs(mut self, out_addrs: Vec<PortAddr>) -> Self {
        self.out_addrs = Some(out_addrs);
        self
    }

    /// Sets the extra information the component will hold.
    ///
    /// # Arguments
    ///
    /// * `extra`: E
    pub fn extra(mut self, extra: E) -> Self {
        self.extra = extra;
        self
    }

    /// Builds the [`Component`].
    pub fn build(self) -> Component<E> {
        // Build dependency map
        // let mut dep_map: Option<Vec<Vec<usize>>> = None;
        // if let Some(sub_comps) = &self.sub_comps {
        //     let mut map = vec![vec![]; sub_comps.len()];
        //     if let Some(connections) = &self.connections {
        //         for conn in connections {
        //             map[idx_of(conn.to)].push(idx_of(conn.from));
        //         }
        //     }
        //     dep_map = Some(map);
        // }

        let mut sub = None;
        if let Some(sub_comps) = self.sub_comps {
            let mut used_inputs: Vec<Vec<bool>> =
                sub_comps.iter().map(|c| vec![false; c.inputs]).collect();

            let connections = self.connections.unwrap_or_default();
            for conn in connections.iter() {
                if used_inputs[idx_of(conn.to)][addr_of(conn.to)] {
                    panic!(
                        "[{0}] Input {1} of comp {2} has two entries (one from [{3}; {4}])",
                        self.id,
                        addr_of(conn.to),
                        idx_of(conn.to),
                        idx_of(conn.from),
                        addr_of(conn.from)
                    );
                }
                used_inputs[idx_of(conn.to)][addr_of(conn.to)] = true;
            }

            let in_addrs = self.in_addrs.unwrap_or_default();
            for (in_idx, (comp_idx, addr)) in in_addrs.iter() {
                if used_inputs[*comp_idx][*addr] {
                    panic!(
                        "[{0}] Input {1} of comp {2} has two entries (one from input {3})",
                        self.id, addr, comp_idx, in_idx,
                    );
                }
                used_inputs[*comp_idx][*addr] = true;
            }

            sub = Some(SubComponent {
                components: sub_comps,
                connections,
                in_addrs,
                out_addrs: self.out_addrs.unwrap_or_default(),
            });
        }

        Component {
            id: self.id,
            name: self.name,
            inputs: self.inputs,
            outputs: self.outputs,
            extra: self.extra,
            sub,
        }
    }
}
