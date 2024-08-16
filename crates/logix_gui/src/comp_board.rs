use egui::Pos2;
use logix_core::component::{Component, Conn, PortAddr};
use logix_sim::primitives::primitives::{ExtraInfo, Primitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConnectionInfo {
    pub points: Vec<Pos2>,
}

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
}

impl ComponentBoard {
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
}
