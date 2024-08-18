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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentInfo {
    pub id: usize,
    pub name: String,
    pub source: Option<PathBuf>,
    pub primitive: Option<Primitive>,
    pub inputs_name: Vec<String>,
    pub outputs_name: Vec<String>,
    pub inputs_data: Vec<Data>,
    pub outputs_data: Vec<Data>,
}

impl ComponentInfo {
    pub fn build_primitive(&self) -> Result<Component<ExtraInfo>, ()> {
        if self.primitive.is_none() {
            return Err(());
        }

        Ok(Component {
            id: self.id,
            name: Some(self.name.clone()),
            inputs: self.inputs_data.len(),
            outputs: self.outputs_data.len(),
            sub: None,
            extra: ExtraInfo {
                id: self.id,
                primitive: Some(self.primitive.clone().unwrap()),
            },
        })
    }

    pub fn build_component(&self) -> Result<Component<ExtraInfo>, ()> {
        if self.primitive.is_some() {
            return self.build_primitive();
        }

        assert!(self.source.is_some());
        let source = self.source.clone().unwrap();
        let serialized = match std::fs::read_to_string(&source) {
            Ok(serialized) => serialized,
            Err(_) => return Err(()),
        };

        let board: ComponentBoard = match serde_json::from_str(&serialized) {
            Ok(board) => board,
            Err(_) => return Err(()),
        };

        board.build_component()
    }

    // pub fn from_source(id: usize, source: PathBuf) -> Result<Self, ()> {
    //     let serialized = match std::fs::read_to_string(&source) {
    //         Ok(serialized) => serialized,
    //         Err(_) => return Err(()),
    //     };
    //     let board: ComponentBoard = match serde_json::from_str(&serialized) {
    //         Ok(board) => board,
    //         Err(_) => return Err(()),
    //     };

    //     Ok(board.board_info(id, Some(source)))
    // }

    pub fn input_count(&self) -> usize {
        self.inputs_name.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs_name.len()
    }

    pub fn and_gate(id: usize, in_count: usize) -> Self {
        ComponentInfo {
            id,
            name: "AND".to_string(),
            source: None,
            primitive: Some(Primitive::AndGate),
            inputs_name: (0..in_count).map(|_| Default::default()).collect(),
            outputs_name: vec![Default::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn nand_gate(id: usize, in_count: usize) -> Self {
        ComponentInfo {
            id,
            name: "NAND".to_string(),
            source: None,
            primitive: Some(Primitive::NandGate),
            inputs_name: (0..in_count).map(|_| Default::default()).collect(),
            outputs_name: vec![Default::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn or_gate(id: usize, in_count: usize) -> Self {
        ComponentInfo {
            id,
            name: "OR".to_string(),
            source: None,
            primitive: Some(Primitive::OrGate),
            inputs_name: (0..in_count).map(|_| Default::default()).collect(),
            outputs_name: vec![Default::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn nor_gate(id: usize, in_count: usize) -> Self {
        ComponentInfo {
            id,
            name: "NOR".to_string(),
            source: None,
            primitive: Some(Primitive::NorGate),
            inputs_name: (0..in_count).map(|_| Default::default()).collect(),
            outputs_name: vec![Default::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn xor_gate(id: usize, in_count: usize) -> Self {
        ComponentInfo {
            id,
            name: "XOR".to_string(),
            source: None,
            primitive: Some(Primitive::XorGate),
            inputs_name: (0..in_count).map(|_| Default::default()).collect(),
            outputs_name: vec![Default::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn not_gate(id: usize) -> Self {
        ComponentInfo {
            id,
            name: "NOT".to_string(),
            source: None,
            primitive: Some(Primitive::NotGate),
            inputs_name: vec![Default::default()],
            outputs_name: vec![Default::default()],
            inputs_data: vec![Data::low()],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn const_high_gate(id: usize) -> Self {
        ComponentInfo {
            id,
            name: "HIGH".to_string(),
            source: None,
            primitive: Some(Primitive::Const {
                value: Data::high(),
            }),
            inputs_name: vec![],
            outputs_name: vec![Default::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::high()],
        }
    }

    pub fn const_low_gate(id: usize) -> Self {
        ComponentInfo {
            id,
            name: "LOW".to_string(),
            source: None,
            primitive: Some(Primitive::Const { value: Data::low() }),
            inputs_name: vec![],
            outputs_name: vec![Default::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn clock_gate(id: usize) -> Self {
        ComponentInfo {
            id,
            name: "CLK".to_string(),
            source: None,
            primitive: Some(Primitive::Clock { period: 1000000000 }),
            inputs_name: vec![],
            outputs_name: vec![Default::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn splitter(id: usize, bits: u8) -> Self {
        ComponentInfo {
            id,
            name: "SPLIT".to_string(),
            source: None,
            primitive: Some(Primitive::Splitter { bits }),
            inputs_name: vec![Default::default()],
            outputs_name: (0..bits).map(|b| b.to_string()).collect(),
            inputs_data: vec![Data::low()],
            outputs_data: vec![Data::low(); bits as usize],
        }
    }

    pub fn joiner(id: usize, bits: u8) -> Self {
        ComponentInfo {
            id,
            name: "JOIN".to_string(),
            source: None,
            primitive: Some(Primitive::Joiner { bits }),
            inputs_name: (0..bits).map(|b| b.to_string()).collect(),
            outputs_name: vec![Default::default()],
            inputs_data: vec![Data::low(); bits as usize],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn input(id: usize, bits: u8) -> Self {
        ComponentInfo {
            id,
            name: "IN".to_string(),
            source: None,
            primitive: Some(Primitive::Input { bits }),
            inputs_name: vec![],
            outputs_name: vec![Default::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn output(id: usize, bits: u8) -> Self {
        ComponentInfo {
            id,
            name: "OUT".to_string(),
            source: None,
            primitive: Some(Primitive::Output { bits }),
            inputs_name: vec![Default::default()],
            outputs_name: vec![],
            inputs_data: vec![Data::low()],
            outputs_data: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentBoard {
    pub name: String,
    pub inputs: usize,
    pub outputs: usize,
    pub comp_pos: Vec<Pos2>,
    pub comp_conns: Vec<ConnectionInfo>,

    pub components: Vec<ComponentInfo>,

    pub inputs_idx: Vec<usize>,
    pub outputs_idx: Vec<usize>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<(usize, PortAddr)>,
    pub out_addrs: Vec<PortAddr>,

    pub inputs_name: Vec<String>,
    pub outputs_name: Vec<String>,
}

impl ComponentBoard {
    pub fn build_component(&self) -> Result<Component<ExtraInfo>, ()> {
        let sub_comps: Result<Vec<Component<ExtraInfo>>, ()> = self
            .components
            .iter()
            .map(|c| c.build_component())
            .collect();

        let sub: SubComponent<ExtraInfo> = match sub_comps {
            Ok(sub_comps) => SubComponent {
                components: sub_comps,
                connections: self.connections.clone(),
                in_addrs: self.in_addrs.clone(),
                out_addrs: self.out_addrs.clone(),
            },
            Err(_) => return Err(()),
        };

        Ok(Component {
            id: 0,
            name: Some(self.name.clone()),
            inputs: self.inputs,
            outputs: self.outputs,
            sub: Some(sub),
            extra: ExtraInfo {
                id: 0,
                primitive: None,
            },
        })
    }

    pub fn board_info(&self, id: usize, source: Option<PathBuf>) -> ComponentInfo {
        ComponentInfo {
            id,
            name: self.name.clone(),
            source: source,
            primitive: None,
            inputs_name: self.inputs_name.clone(),
            outputs_name: self.outputs_name.clone(),
            inputs_data: self
                .inputs_idx
                .iter()
                .map(|i| self.components[*i].outputs_data[0])
                .collect(),
            outputs_data: self
                .outputs_idx
                .iter()
                .map(|i| self.components[*i].inputs_data[0])
                .collect(),
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

    pub fn add_comp(&mut self, subc: ComponentInfo, pos: Pos2) {
        self.components.push(subc);
        self.comp_pos.push(pos);

        if let Some(prim) = self.components.last().unwrap().primitive.clone() {
            match prim {
                Primitive::Input { bits: _ } => {
                    self.inputs += 1;
                    self.inputs_idx.push(self.components.len() - 1);
                    self.inputs_name.push(Default::default());
                }
                Primitive::Output { bits: _ } => {
                    self.outputs += 1;
                    self.outputs_idx.push(self.components.len() - 1);
                    self.outputs_name.push(Default::default());
                }
                _ => {}
            }
        }
    }

    pub fn import_comp(&mut self, id: usize, source: PathBuf, pos: Pos2) -> Result<(), ()> {
        let serialized = match std::fs::read_to_string(&source) {
            Ok(serialized) => serialized,
            Err(_) => return Err(()),
        };

        let board: ComponentBoard = match serde_json::from_str(&serialized) {
            Ok(board) => board,
            Err(_) => return Err(()),
        };

        let comp = board.board_info(id, Some(source));
        self.add_comp(comp, pos);
        Ok(())
    }

    pub fn remove_comp(&mut self, idx: usize) {
        self.components.remove(idx);
        self.comp_pos.remove(idx);

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

        if let Some(prim) = &self.components[from].primitive {
            if prim.is_input() {
                let from_input = self.inputs_idx.iter().position(|&x| x == from).unwrap();
                self.in_addrs.push((from_input, (to, to_port)));
            }
        }

        if let Some(prim) = &self.components[to].primitive {
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
        if let Some(prim) = &self.components[conn.from.0].primitive {
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
        if let Some(prim) = &self.components[conn.to.0].primitive {
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
        let and_gate = ComponentInfo::and_gate(id, in_count);
        self.add_comp(and_gate, pos);
    }

    pub fn add_nand_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nand_gate = ComponentInfo::nand_gate(id, in_count);
        self.add_comp(nand_gate, pos);
    }

    pub fn add_or_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let or_gate = ComponentInfo::or_gate(id, in_count);
        self.add_comp(or_gate, pos);
    }

    pub fn add_nor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nor_gate = ComponentInfo::nor_gate(id, in_count);
        self.add_comp(nor_gate, pos);
    }

    pub fn add_xor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let xor_gate = ComponentInfo::xor_gate(id, in_count);
        self.add_comp(xor_gate, pos);
    }

    pub fn add_not_gate(&mut self, id: usize, pos: Pos2) {
        let not_gate = ComponentInfo::not_gate(id);
        self.add_comp(not_gate, pos);
    }

    pub fn add_const_high_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = ComponentInfo::const_high_gate(id);
        self.add_comp(const_gate, pos);
    }

    pub fn add_const_low_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = ComponentInfo::const_low_gate(id);
        self.add_comp(const_gate, pos);
    }

    pub fn add_clock_gate(&mut self, id: usize, pos: Pos2) {
        let clock_gate = ComponentInfo::clock_gate(id);
        self.add_comp(clock_gate, pos);
    }

    pub fn add_splitter(&mut self, id: usize, bits: u8, pos: Pos2) {
        let splitter = ComponentInfo::splitter(id, bits);
        self.add_comp(splitter, pos);
    }

    pub fn add_joiner(&mut self, id: usize, bits: u8, pos: Pos2) {
        let joiner = ComponentInfo::joiner(id, bits);
        self.add_comp(joiner, pos);
    }

    pub fn add_input(&mut self, id: usize, bits: u8, pos: Pos2) {
        let input = ComponentInfo::input(id, bits);
        self.add_comp(input, pos);
    }

    pub fn add_output(&mut self, id: usize, bits: u8, pos: Pos2) {
        let output = ComponentInfo::output(id, bits);
        self.add_comp(output, pos);
    }
}
