use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::data::Data;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PASM {
    pub cmds: Vec<PASMCommand>,
    pub update_type: ProgramUpdateType,
    pub state: PASMProgramState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgramUpdateType {
    InputChanges,
    Always,
}

const FLAG_EQUAL: usize = 0;
const FLAG_LESS: usize = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PASMProgramState {
    // Declared variables
    pub vars: HashMap<String, Data>,

    // Program counter
    pub pc: usize,

    // Flags
    //  bit\val   0        1
    //    0       equal    not equal
    //    1       greater  less
    pub flags: usize,

    // Line position of labels
    pub label_pos: HashMap<String, usize>,

    // Time to wait from
    pub waiting_from: Option<u128>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PASMExpr {
    Not(Box<PASMExpr>),
    And(Vec<PASMExpr>),
    Or(Vec<PASMExpr>),
    Nand(Vec<PASMExpr>),
    Nor(Vec<PASMExpr>),
    Xor(Vec<PASMExpr>),
    BitVec(Vec<PASMExpr>),
    Var(String),
    Const(Data),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PASMCommand {
    // Move a value into a variable
    Mov { name: String, value: PASMExpr },
    // Declares a label
    Label { name: String },
    // Goto a label
    Goto { label: String },
    // Compares two values (this sets the flags)
    Cmp { v1: PASMExpr, v2: PASMExpr },
    // Jumps to a label if the cmp flags was 'equal'
    Je { label: String },
    // Jumps to a label if the cmp flags was 'not equal'
    Jne { label: String },
    // Jumps to a label if the cmp flags was 'greater than'
    Jg { label: String },
    // Jumps to a label if the cmp flags was 'less than'
    Jl { label: String },
    // Jumps to a label if the cmp flags was 'greater than or equal'
    Jge { label: String },
    // Jumps to a label if the cmp flags was 'less than or equal'
    Jle { label: String },
    // Waits for a certain amount of time (ns)
    Wait { time: u128 },
}

#[macro_export]
macro_rules! pexp {
    (data, $data:expr) => {
        logix_sim::primitives::pasm::PASMExpr::Const($data.into())
    };
    (var, $name:expr) => {
        logix_sim::primitives::pasm::PASMExpr::Var($name.to_string())
    };
    (not, $expr:expr) => {
        logix_sim::primitives::pasm::PASMExpr::Not(Box::new($expr))
    };
    (and, $($expr:expr),*) => {
        logix_sim::primitives::pasm::PASMExpr::And(vec![$($expr),*])
    };
    (or, $($expr:expr),*) => {
        logix_sim::primitives::pasm::PASMExpr::Or(vec![$($expr),*])
    };
    (nand, $($expr:expr),*) => {
        logix_sim::primitives::pasm::PASMExpr::Nand(vec![$($expr),*])
    };
    (nor, $($expr:expr),*) => {
        logix_sim::primitives::pasm::PASMExpr::Nor(vec![$($expr),*])
    };
    (xor, $($expr:expr),*) => {
        logix_sim::primitives::pasm::PASMExpr::Xor(vec![$($expr),*])
    };
    (bit_vec, $($expr:expr),*) => {
        logix_sim::primitives::pasm::PASMExpr::BitVec(vec![$($expr),*])
    };
}

#[macro_export]
macro_rules! pcmd {
    (mov, $name:expr, $val:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Mov {
            name: $name.to_string(),
            value: $val,
        }
    };
    (label, $name:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Label {
            name: $name.to_string(),
        }
    };
    (goto, $label:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Goto {
            label: $label.to_string(),
        }
    };
    (cmp, $v1:expr, $v2:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Cmp { v1: $v1, v2: $v2 }
    };
    (je, $label:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Je {
            label: $label.to_string(),
        }
    };
    (jne, $label:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Jne {
            label: $label.to_string(),
        }
    };
    (jg, $label:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Jg {
            label: $label.to_string(),
        }
    };
    (jl, $label:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Jl {
            label: $label.to_string(),
        }
    };
    (jge, $label:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Jge {
            label: $label.to_string(),
        }
    };
    (jle, $label:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Jle {
            label: $label.to_string(),
        }
    };
    (wait, $time:expr) => {
        logix_sim::primitives::pasm::PASMCommand::Wait { time: $time }
    };
}

impl PASM {
    pub fn new(update_type: ProgramUpdateType, cmds: Vec<PASMCommand>) -> Self {
        let mut label_pos = HashMap::new();
        for (i, cmd) in cmds.iter().enumerate() {
            if let PASMCommand::Label { name } = cmd {
                label_pos.insert(name.clone(), i);
            }
        }
        Self {
            cmds,
            update_type,
            state: PASMProgramState {
                vars: HashMap::new(),
                pc: 0,
                flags: 0,
                label_pos,
                waiting_from: None,
            },
        }
    }

    pub fn with_default_vars(mut self, vars: HashMap<impl Into<String>, impl Into<Data>>) -> Self {
        self.state.vars = vars
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    pub fn set_flag(&mut self, bit: usize, val: bool) {
        if val {
            self.state.flags |= 1 << bit;
        } else {
            self.state.flags &= !(1 << bit);
        }
    }

    pub fn flag_at(&self, bit: usize) -> bool {
        (self.state.flags & (1 << bit)) != 0
    }

    pub fn eval_expr(&mut self, expr: &PASMExpr) -> Data {
        match expr {
            PASMExpr::Not(expr) => !self.eval_expr(expr),
            PASMExpr::And(exprs) => exprs
                .iter()
                .fold(Data::high(), |acc, x| acc & self.eval_expr(x)),
            PASMExpr::Or(exprs) => exprs
                .iter()
                .fold(Data::low(), |acc, x| acc | self.eval_expr(x)),
            PASMExpr::Nand(exprs) => exprs
                .iter()
                .fold(Data::high(), |acc, x| acc & self.eval_expr(x)),
            PASMExpr::Nor(exprs) => exprs
                .iter()
                .fold(Data::low(), |acc, x| acc | self.eval_expr(x)),
            PASMExpr::Xor(exprs) => exprs
                .iter()
                .skip(1)
                .fold(self.eval_expr(&exprs[0]), |acc, x| acc ^ self.eval_expr(x)),
            PASMExpr::Var(name) => self.state.vars[name],
            PASMExpr::Const(value) => *value,
            PASMExpr::BitVec(exprs) => {
                let mut data = Data::new(0, exprs.len() as u8);
                for (i, expr) in exprs.iter().enumerate() {
                    data.set_bit_at(i as u8, self.eval_expr(expr).as_bool());
                }
                data
            }
        }
    }

    pub fn run(&mut self, curr_time: u128) {
        let mut running = true;
        while running {
            let pc = self.state.pc;

            match self.cmds[pc].clone() {
                PASMCommand::Mov { name, value } => {
                    let val = self.eval_expr(&value);
                    self.state.vars.insert(name, val);
                    self.state.pc += 1;
                }
                PASMCommand::Label { .. } => {
                    self.state.pc += 1;
                }
                PASMCommand::Goto { label } => {
                    let pos = self.state.label_pos[&label];
                    self.state.pc = pos;
                }
                PASMCommand::Cmp { v1, v2 } => {
                    let v1 = self.eval_expr(&v1);
                    let v2 = self.eval_expr(&v2);
                    self.set_flag(FLAG_EQUAL, v1 == v2);
                    self.set_flag(FLAG_LESS, v1.value < v2.value);
                    self.state.pc += 1;
                }
                PASMCommand::Je { label } => {
                    if self.flag_at(FLAG_EQUAL) {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PASMCommand::Jne { label } => {
                    if !self.flag_at(FLAG_EQUAL) {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PASMCommand::Jg { label } => {
                    if !self.flag_at(FLAG_LESS) {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PASMCommand::Jl { label } => {
                    if self.flag_at(FLAG_LESS) {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PASMCommand::Jge { label } => {
                    if self.flag_at(FLAG_EQUAL) || !self.flag_at(FLAG_LESS) {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PASMCommand::Jle { label } => {
                    if self.flag_at(FLAG_EQUAL) || self.flag_at(FLAG_LESS) {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PASMCommand::Wait { time } => {
                    if let Some(from) = self.state.waiting_from {
                        if from + time <= curr_time {
                            self.state.waiting_from = None;
                            self.state.pc += 1;
                        } else {
                            running = false;
                        }
                    } else {
                        self.state.waiting_from = Some(curr_time);
                        running = false;
                    }
                }
            }

            if pc >= self.cmds.len() - 1 {
                // Just start again in the next update (run)
                self.state.pc = 0;
                running = false;
            }
        }
    }
}
