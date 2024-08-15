use egui::Pos2;
use logix_core::component::{Component, Conn, SubComponent};
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
    pub board: SubComponent<ExtraInfo>,
    pub subc_pos: Vec<Pos2>,
    pub subc_conns: Vec<ConnectionInfo>,
}

impl ComponentBoard {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn add_subc(&mut self, subc: Component<ExtraInfo>, pos: Pos2) {
        self.board.components.push(subc);
        self.subc_pos.push(pos);
    }

    pub fn remove_subc(&mut self, idx: usize) {
        self.board.components.remove(idx);
        self.subc_pos.remove(idx);

        // Remove input connections to the subcomponent
        for i in 0..self.board.in_addrs.len() {
            if self.board.in_addrs[i].1 .0 == idx {
                self.board.in_addrs.remove(i);
                self.inputs -= 1;
            }
        }

        // Remove output connections from the subcomponent
        for i in 0..self.board.out_addrs.len() {
            if self.board.out_addrs[i].0 == idx {
                self.board.out_addrs.remove(i);
                self.outputs -= 1;
            }
        }

        // Update connections
        let l = self.board.connections.len();
        let conns = &mut self.board.connections;
        for i in 0..l {
            let conn = conns[i];

            // Remove connections related to the subcomponent
            if conn.from.0 == idx || conn.to.0 == idx {
                conns.remove(i);
            }

            // Update forward connections indices
            if conn.from.0 > idx {
                conns[i].from.0 -= 1;
            }
            if conn.to.0 > idx {
                conns[i].to.0 -= 1;
            }
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
        self.board.connections.push(Conn {
            from: (from, from_port),
            to: (to, to_port),
        });
        self.subc_conns.push(ConnectionInfo { points });
    }

    pub fn remove_conn(&mut self, idx: usize) {
        self.board.connections.remove(idx);
        self.subc_conns.remove(idx);
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
