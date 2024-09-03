use logix_sim::primitives::prelude::Primitive;

use super::{board_comp::BoardComponent, board_conn::BoardConnection, io_info::IOInfo, Board};

#[derive(Debug, Clone)]
pub enum BoardAction {
    AddComponent {
        comp: BoardComponent,
    },
    AddConnection {
        conn: BoardConnection,
    },
    RemoveComponent {
        comp: BoardComponent,
        at: usize,
        conns: Vec<(usize, BoardConnection)>,
        input: Option<(usize, IOInfo)>,
        output: Option<(usize, IOInfo)>,
    },
    RemoveConnection {
        conn: BoardConnection,
        at: usize,
    },
}

impl BoardAction {
    pub const fn add_component(comp: BoardComponent) -> Self {
        Self::AddComponent { comp }
    }
    pub const fn add_connection(conn: BoardConnection) -> Self {
        Self::AddConnection { conn }
    }
    pub const fn remove_component(comp: BoardComponent, at: usize) -> Self {
        Self::RemoveComponent {
            comp,
            at,
            conns: vec![],
            input: None,
            output: None,
        }
    }
    pub const fn remove_connection(conn: BoardConnection, at: usize) -> Self {
        Self::RemoveConnection { conn, at }
    }
}

impl Board {
    pub fn not_saved(&self) -> bool {
        self.hist_idx != self.saved_idx
    }

    pub fn undo(&mut self) {
        if self.hist_idx.is_none() {
            return;
        }

        let idx = self.hist_idx.unwrap();
        self.hist_idx = match idx {
            0 => None,
            _ => Some(idx - 1),
        };

        let action = self.hist[idx].clone();
        self._undo_action(action);
    }

    pub fn redo(&mut self) {
        let next_idx = self.hist_idx.map_or(0, |idx| idx + 1);

        if next_idx >= self.hist.len() {
            return;
        }

        self.hist_idx = Some(next_idx);
        let action = self.hist[next_idx].clone();
        self.hist[next_idx] = self._do_action(action);
    }

    fn _do_action(&mut self, mut action: BoardAction) -> BoardAction {
        match &mut action {
            BoardAction::AddComponent { comp } => {
                self.components.push(comp.clone());
                let idx = self.components.len() - 1;
                match comp.info.primitive.as_ref() {
                    Some(Primitive::Input { bits: _ }) => {
                        self.inputs.push(IOInfo::new(idx, String::default()));
                    }
                    Some(Primitive::Output { bits: _ }) => {
                        self.outputs.push(IOInfo::new(idx, String::default()));
                    }
                    _ => {}
                }
            }
            BoardAction::AddConnection { conn } => {
                self.conns.push(conn.clone());
            }
            BoardAction::RemoveComponent {
                comp,
                at,
                conns,
                input,
                output,
            } => {
                let idx = *at;

                self.components.remove(idx);

                if comp.is_input() {
                    let input_idx = self
                        .inputs
                        .iter()
                        .position(|input| input.idx == idx)
                        .unwrap();
                    *input = Some((input_idx, self.inputs.remove(input_idx)));
                }
                if comp.is_output() {
                    let output_idx = self
                        .outputs
                        .iter()
                        .position(|output| output.idx == idx)
                        .unwrap();
                    *output = Some((output_idx, self.outputs.remove(output_idx)));
                }

                // Update indices in inputs/outputs according to the removed component
                self.inputs.iter_mut().for_each(|input| {
                    if input.idx > idx {
                        input.idx -= 1;
                    }
                });
                self.outputs.iter_mut().for_each(|output| {
                    if output.idx > idx {
                        output.idx -= 1;
                    }
                });

                // Remove connections related to the removed component
                *conns = self
                    .conns
                    .iter()
                    .enumerate()
                    .filter_map(|(i, conn)| {
                        if conn.conn.from.0 == idx || conn.conn.to.0 == idx {
                            Some((i, conn.clone()))
                        } else {
                            None
                        }
                    })
                    .collect();

                self.conns
                    .retain(|conn| conn.conn.from.0 != idx && conn.conn.to.0 != idx);

                // Update indices in connections according to the removed component
                self.conns.iter_mut().for_each(|conn| {
                    if conn.conn.from.0 > idx {
                        conn.conn.from.0 -= 1;
                    }
                    if conn.conn.to.0 > idx {
                        conn.conn.to.0 -= 1;
                    }
                });
            }
            BoardAction::RemoveConnection { conn: _, at } => {
                self.conns.remove(*at);
            }
        }
        action
    }

    fn _undo_action(&mut self, action: BoardAction) {
        match action {
            BoardAction::AddComponent { .. } => {
                self.components.pop();
                let idx = self.components.len();

                self.inputs.retain(|input| input.idx != idx);
                self.outputs.retain(|output| output.idx != idx);
            }
            BoardAction::AddConnection { .. } => {
                self.conns.pop();
            }
            BoardAction::RemoveComponent {
                comp,
                at,
                conns,
                input,
                output,
            } => {
                for input in &mut self.inputs {
                    if input.idx >= at {
                        input.idx += 1;
                    }
                }

                for output in &mut self.outputs {
                    if output.idx >= at {
                        output.idx += 1;
                    }
                }

                if let Some((input_idx, input)) = input {
                    self.inputs.insert(input_idx, input);
                }

                if let Some((output_idx, output)) = output {
                    self.outputs.insert(output_idx, output);
                }

                for conn in &mut self.conns {
                    if conn.conn.from.0 >= at {
                        conn.conn.from.0 += 1;
                    }
                    if conn.conn.to.0 >= at {
                        conn.conn.to.0 += 1;
                    }
                }

                for (i, conn) in conns {
                    self.conns.insert(i, conn);
                }

                self.components.insert(at, comp);
            }
            BoardAction::RemoveConnection { conn, at } => {
                self.conns.insert(at, conn);
            }
        }
    }

    pub fn execute(&mut self, action: BoardAction) {
        self.hist.truncate(self.hist_idx.unwrap_or(0));
        self.hist.push(action.clone());
        let next_idx = self.hist.len() - 1;
        self.hist_idx = Some(next_idx);

        self.hist[next_idx] = self._do_action(action);
    }
}
