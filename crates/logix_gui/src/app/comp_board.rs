use std::path::PathBuf;

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
    pub components_source: Vec<PathBuf>,

    pub inputs_order: Vec<usize>,
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

    pub fn save(&self, path: &PathBuf) -> Result<(), ()> {
        let serialized = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(_) => return Err(()),
        };
        match std::fs::write(path, serialized) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self, ()> {
        let serialized = match std::fs::read_to_string(path) {
            Ok(serialized) => serialized,
            Err(_) => return Err(()),
        };
        match serde_json::from_str(&serialized) {
            Ok(board) => Ok(board),
            Err(_) => Err(()),
        }
    }

    fn get_default_data_vals(comp: &Component<ExtraInfo>) -> (Vec<Data>, Vec<Data>) {
        (
            (0..comp.inputs)
                .map(|_| {
                    if let Some(prim) = comp.extra.primitive.clone() {
                        prim.input_default_data().expect("Invalid primitive")
                    } else {
                        // Use later for complex components
                        Data::low()
                    }
                })
                .collect(),
            (0..comp.outputs)
                .map(|_| {
                    if let Some(prim) = comp.extra.primitive.clone() {
                        prim.output_default_data().expect("Invalid primitive")
                    } else {
                        // Use later for complex components
                        Data::low()
                    }
                })
                .collect(),
        )
    }

    pub fn add_comp(&mut self, subc: Component<ExtraInfo>, pos: Pos2) {
        self.components.push(subc);
        self.comp_pos.push(pos);
        self.data_vals.push(Self::get_default_data_vals(
            &self.components.last().unwrap(),
        ));

        if let Some(prim) = self.components.last().unwrap().extra.primitive.clone() {
            match prim {
                Primitive::Input { bits: _ } => {
                    self.inputs += 1;
                    self.inputs_order.push(self.components.len() - 1);
                }
                Primitive::Output { bits: _ } => {
                    self.outputs += 1;
                }
                _ => {}
            }
        }
    }

    pub fn remove_comp(&mut self, idx: usize) {
        self.components.remove(idx);
        self.comp_pos.remove(idx);
        self.data_vals.remove(idx);

        // Remove input connections to the component
        for i in 0..self.in_addrs.len() {
            if self.in_addrs[i].1 .0 == idx {
                self.in_addrs.remove(i);
                self.inputs -= 1;
            }
        }

        // Remove output connections from the component
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

            // Remove connections related to the component
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

        if let Some(prim) = &self.components[from].extra.primitive {
            if prim.is_input() {
                let from_input = self.inputs_order.iter().position(|&x| x == from).unwrap();
                self.in_addrs.push((from_input, (to, to_port)));
            }
        }

        if let Some(prim) = &self.components[to].extra.primitive {
            if prim.is_output() {
                self.out_addrs.push((from, from_port));
            }
        }
    }

    pub fn remove_conn(&mut self, idx: usize) {
        let conn = self.connections[idx].clone();
        self.connections.remove(idx);
        self.comp_conns.remove(idx);

        // Check if connection is an input connection
        if let Some(prim) = &self.components[conn.from.0].extra.primitive {
            if prim.is_input() {
                let mut i = 0;
                while i < self.in_addrs.len() {
                    if self.in_addrs[i].0 == conn.from.0 && self.in_addrs[i].1 == conn.to {
                        self.in_addrs.remove(i);
                        break;
                    }
                    i += 1;
                }
            }
        }

        // Check if connection is an output connection
        if let Some(prim) = &self.components[conn.to.0].extra.primitive {
            if prim.is_output() {
                let mut i = 0;
                while i < self.out_addrs.len() {
                    if self.out_addrs[i] == conn.from {
                        self.out_addrs.remove(i);
                        break;
                    }
                    i += 1;
                }
            }
        }
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
        self.add_comp(and_gate, pos);
    }

    pub fn add_nand_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let and_gate = Component {
            id,
            name: Some("NAND".to_string()),
            inputs: in_count,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::NandGate),
            sub: None,
        };
        self.add_comp(and_gate, pos);
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
        self.add_comp(or_gate, pos);
    }

    pub fn add_nor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nor_gate = Component {
            id,
            name: Some("NOR".to_string()),
            inputs: in_count,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::NorGate),
            sub: None,
        };
        self.add_comp(nor_gate, pos);
    }

    pub fn add_xor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let xor_gate = Component {
            id,
            name: Some("XOR".to_string()),
            inputs: in_count,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::XorGate),
            sub: None,
        };
        self.add_comp(xor_gate, pos);
    }

    pub fn add_not_gate(&mut self, id: usize, pos: Pos2) {
        let xnor_gate = Component {
            id,
            name: Some("NOT".to_string()),
            inputs: 1,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::NotGate),
            sub: None,
        };
        self.add_comp(xnor_gate, pos);
    }

    pub fn add_const_high_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = Component {
            id,
            name: Some("HIGH".to_string()),
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
        self.add_comp(const_gate, pos);
    }

    pub fn add_const_low_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = Component {
            id,
            name: Some("LOW".to_string()),
            inputs: 0,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::Const { value: Data::low() }),
            sub: None,
        };
        self.add_comp(const_gate, pos);
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
        self.add_comp(clock_gate, pos);
    }

    pub fn add_splitter(&mut self, id: usize, bits: u8, pos: Pos2) {
        let splitter = Component {
            id,
            name: Some("SPLIT".to_string()),
            inputs: 1,
            outputs: bits as usize,
            extra: ExtraInfo::from_primitive(id, Primitive::Splitter { bits }),
            sub: None,
        };
        self.add_comp(splitter, pos);
    }

    pub fn add_joiner(&mut self, id: usize, bits: u8, pos: Pos2) {
        let joiner = Component {
            id,
            name: Some("JOIN".to_string()),
            inputs: bits as usize,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::Joiner { bits }),
            sub: None,
        };
        self.add_comp(joiner, pos);
    }

    pub fn add_input(&mut self, id: usize, bits: u8, pos: Pos2) {
        let input = Component {
            id,
            name: Some("IN".to_string()),
            inputs: 0,
            outputs: 1,
            extra: ExtraInfo::from_primitive(id, Primitive::Input { bits }),
            sub: None,
        };
        self.add_comp(input, pos);
    }

    pub fn add_output(&mut self, id: usize, bits: u8, pos: Pos2) {
        let output = Component {
            id,
            name: Some("OUT".to_string()),
            inputs: 1,
            outputs: 0,
            extra: ExtraInfo::from_primitive(id, Primitive::Output { bits }),
            sub: None,
        };
        self.add_comp(output, pos);
    }
}
