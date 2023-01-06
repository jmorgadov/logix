pub type PortAddr = (usize, usize);

#[inline(always)]
pub fn idx_of(port_addr: PortAddr) -> usize {
    port_addr.0
}

#[inline(always)]
pub fn addr_of(port_addr: PortAddr) -> usize {
    port_addr.1
}

#[derive(Default, Debug)]
pub struct Conn {
    pub from: PortAddr,
    pub to: PortAddr,
}

impl Conn {
    pub fn new(from_idx: usize, from_port: usize, to_idx: usize, to_port: usize) -> Conn {
        Conn {
            from: (from_idx, from_port),
            to: (to_idx, to_port),
        }
    }
}

#[derive(Default, Debug)]
pub struct SubComponent {
    pub components: Vec<Component>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<PortAddr>,
    pub out_addrs: Vec<PortAddr>,
    pub dep_map: Vec<Vec<usize>>,
}

#[derive(Default, Debug)]
pub struct Component {
    pub name: String,
    pub inputs: Vec<bool>,
    pub outputs: Vec<bool>,

    pub sub: Option<SubComponent>,
    pub info: Vec<u8>,
}

#[derive(Default)]
pub struct ComponentBuilder {
    name: String,
    inputs: Vec<bool>,
    outputs: Vec<bool>,

    sub_comps: Option<Vec<Component>>,
    connections: Option<Vec<Conn>>,
    in_addrs: Option<Vec<PortAddr>>,
    out_addrs: Option<Vec<PortAddr>>,
    info: Vec<u8>,
}

impl ComponentBuilder {
    pub fn new(name: &str) -> Self {
        ComponentBuilder {
            name: name.to_string(),
            inputs: vec![],
            outputs: vec![],
            sub_comps: None,
            connections: None,
            in_addrs: None,
            out_addrs: None,
            info: vec![],
        }
    }

    pub fn in_count(mut self, n: usize) -> Self {
        self.inputs = vec![false; n];
        self
    }

    pub fn out_count(mut self, n: usize) -> Self {
        self.outputs = vec![false; n];
        self
    }

    pub fn port_count(mut self, in_count: usize, out_count: usize) -> Self {
        self.inputs = vec![false; in_count];
        self.outputs = vec![false; out_count];
        self
    }

    pub fn sub_comps(mut self, sub_comps: Vec<Component>) -> Self {
        self.sub_comps = Some(sub_comps);
        self
    }

    pub fn connections(mut self, connections: Vec<Conn>) -> Self {
        self.connections = Some(connections);
        self
    }

    pub fn in_addrs(mut self, in_addrs: Vec<PortAddr>) -> Self {
        self.in_addrs = Some(in_addrs);
        self
    }

    pub fn out_addrs(mut self, out_addrs: Vec<PortAddr>) -> Self {
        self.out_addrs = Some(out_addrs);
        self
    }

    pub fn info(mut self, info: Vec<u8>) -> Self {
        self.info = info;
        self
    }

    pub fn build(self) -> Component {
        // Build dependency map
        let mut dep_map: Option<Vec<Vec<usize>>> = None;
        if let Some(sub_comps) = &self.sub_comps {
            let mut map = vec![vec![]; sub_comps.len()];
            if let Some(connections) = &self.connections {
                for conn in connections {
                    map[idx_of(conn.to)].push(idx_of(conn.from));
                }
            }
            dep_map = Some(map);
        }

        let mut sub = None;
        if let Some(sub_comps) = self.sub_comps {
            sub = Some(SubComponent {
                components: sub_comps,
                connections: self.connections.unwrap_or_default(),
                in_addrs: self.in_addrs.unwrap_or_default(),
                out_addrs: self.out_addrs.unwrap_or_default(),
                dep_map: dep_map.unwrap_or_default(),
            })
        }

        Component {
            name: self.name,
            inputs: self.inputs,
            outputs: self.outputs,
            info: self.info,
            sub,
        }
    }
}
