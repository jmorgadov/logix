use egui::Pos2;
use logix_core::component::{Component, Conn, PortAddr, SubComponent};
use logix_sim::primitives::{
    data::Data,
    primitives::{ExtraInfo, Primitive},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConnectionInfo {
    pub points: Vec<Pos2>,
}

pub type ComponentsData = Vec<(Vec<Data>, Vec<Data>)>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentBoard {
    pub name: String,
    pub inputs: usize,
    pub outputs: usize,
    pub comp_pos: Vec<Pos2>,
    pub comp_conns: Vec<ConnectionInfo>,

    pub components: Vec<Component<ExtraInfo>>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<(usize, PortAddr)>,
    pub out_addrs: Vec<PortAddr>,

    pub data_vals: ComponentsData,
}

impl ComponentBoard {
    pub fn build_component(&mut self) -> Component<ExtraInfo> {
        Component {
            id: 0,
            name: Some(self.name.clone()),
            inputs: self.inputs,
            outputs: self.outputs,
            sub: Some(SubComponent {
                components: self.components.clone(),
                connections: self.connections.clone(),
                in_addrs: self.in_addrs.clone(),
                out_addrs: self.out_addrs.clone(),
            }),
            ..Default::default()
        }
    }

    pub fn build_data_vals(&mut self) {
        self.data_vals = self
            .components
            .iter()
            .map(|comp| {
                (
                    (0..comp.inputs).map(|_| Data::low()).collect(),
                    (0..comp.outputs).map(|_| Data::low()).collect(),
                )
            })
            .collect();
    }

    pub fn add_subc(&mut self, subc: Component<ExtraInfo>, pos: Pos2) {
        self.components.push(subc);
        self.comp_pos.push(pos);
    }

    pub fn remove_subc(&mut self, idx: usize) {
        self.components.remove(idx);
        self.comp_pos.remove(idx);

        // Remove input connections to the subcomponent
        for i in 0..self.in_addrs.len() {
            if self.in_addrs[i].1 .0 == idx {
                self.in_addrs.remove(i);
                self.inputs -= 1;
            }
        }

        // Remove output connections from the subcomponent
        for i in 0..self.out_addrs.len() {
            if self.out_addrs[i].0 == idx {
                self.out_addrs.remove(i);
                self.outputs -= 1;
            }
        }

        // Update connections
        let mut i = 0;
        while i < self.connections.len() {
            let conn = self.connections[i];

            // Remove connections related to the subcomponent
            if conn.from.0 == idx || conn.to.0 == idx {
                self.connections.remove(i);
                self.comp_conns.remove(i);
                continue;
            }

            // Update forward connections indices
            if conn.from.0 > idx {
                self.connections[i].from.0 -= 1;
            }
            if conn.to.0 > idx {
                self.connections[i].to.0 -= 1;
            }
            i += 1;
        }
    }

    pub fn add_conn(
        &mut self,
        from: usize,
        to: usize,
        from_port: usize,
        to_port: usize,
        points: Vec<Pos2>,
    ) {
        self.connections.push(Conn {
            from: (from, from_port),
            to: (to, to_port),
        });
        self.comp_conns.push(ConnectionInfo { points });
    }

    pub fn remove_conn(&mut self, idx: usize) {
        self.connections.remove(idx);
        self.comp_conns.remove(idx);
    }

    pub fn add_and_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let and_gate = Component {
            id,
            name: Some("AND".to_string()),
            inputs: in_count,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::AndGate),
            sub: None,
        };
        self.add_subc(and_gate, pos);
    }

    pub fn add_or_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let or_gate = Component {
            id,
            name: Some("OR".to_string()),
            inputs: in_count,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::OrGate),
            sub: None,
        };
        self.add_subc(or_gate, pos);
    }

    pub fn add_const_high_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = Component {
            id,
            name: Some("CONST".to_string()),
            inputs: 0,
            outputs: 1,
            extra: ExtraInfo::from_primitive(
                id,
                Primitive::Const {
                    value: Data::high(),
                },
            ),
            sub: None,
        };
        self.add_subc(const_gate, pos);
    }

    pub fn add_const_low_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = Component {
            id,
            name: Some("CONST".to_string()),
            inputs: 0,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::Const { value: Data::low() }),
            sub: None,
        };
        self.add_subc(const_gate, pos);
    }

    pub fn add_clock_gate(&mut self, id: usize, pos: Pos2) {
        let clock_gate = Component {
            id,
            name: Some("CLK".to_string()),
            inputs: 0,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::Clock { period: 1000000000 }),
            sub: None,
        };
        self.add_subc(clock_gate, pos);
    }
}
