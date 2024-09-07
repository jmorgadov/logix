use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::data::Data;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimProgram {
    pub cmds: Vec<PrimCommand>,
    pub update_type: ProgramUpdateType,
    pub state: PrimProgramState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgramUpdateType {
    InputChanges,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimProgramState {
    // Declared variables
    pub vars: HashMap<String, Data>,

    // Program counter
    pub pc: usize,

    // Comparison flags
    //  -1: less than
    //  0: equal
    //  1: greater
    pub cmp_flag: i8,

    // Line position of labels
    pub label_pos: HashMap<String, usize>,

    // Time to wait from
    pub waiting_from: Option<u128>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimExpr {
    Not(Box<PrimExpr>),
    And(Vec<PrimExpr>),
    Or(Vec<PrimExpr>),
    Nand(Vec<PrimExpr>),
    Nor(Vec<PrimExpr>),
    Xor(Vec<PrimExpr>),
    BitVec(Vec<PrimExpr>),
    Var(String),
    Const(Data),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimCommand {
    // Move a value into a variable
    Mov { name: String, value: PrimExpr },
    // Declares a label
    Label { name: String },
    // Goto a label
    Goto { label: String },
    // Compares a value to a variable (this sets the flags)
    CmpVar { name: String, value: PrimExpr },
    // Compares two values (this sets the flags)
    Cmp { v1: PrimExpr, v2: PrimExpr },
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
        logix_sim::primitives::prim_program::PrimExpr::Const($data.into())
    };
    (var, $name:expr) => {
        logix_sim::primitives::prim_program::PrimExpr::Var($name.to_string())
    };
    (not, $expr:expr) => {
        logix_sim::primitives::prim_program::PrimExpr::Not(Box::new($expr))
    };
    (and, $($expr:expr),*) => {
        logix_sim::primitives::prim_program::PrimExpr::And(vec![$($expr),*])
    };
    (or, $($expr:expr),*) => {
        logix_sim::primitives::prim_program::PrimExpr::Or(vec![$($expr),*])
    };
    (nand, $($expr:expr),*) => {
        logix_sim::primitives::prim_program::PrimExpr::Nand(vec![$($expr),*])
    };
    (nor, $($expr:expr),*) => {
        logix_sim::primitives::prim_program::PrimExpr::Nor(vec![$($expr),*])
    };
    (xor, $($expr:expr),*) => {
        logix_sim::primitives::prim_program::PrimExpr::Xor(vec![$($expr),*])
    };
    (bit_vec, $($expr:expr),*) => {
        logix_sim::primitives::prim_program::PrimExpr::BitVec(vec![$($expr),*])
    };
}

#[macro_export]
macro_rules! pcmd {
    (mov, $name:expr, $val:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Mov {
            name: $name.to_string(),
            value: $val,
        }
    };
    (label, $name:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Label {
            name: $name.to_string(),
        }
    };
    (goto, $label:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Goto {
            label: $label.to_string(),
        }
    };
    (cmp_var, $name:expr, $val:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::CmpVar {
            name: $name.to_string(),
            value: $val,
        }
    };
    (cmp, $v1:expr, $v2:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Cmp { v1: $v1, v2: $v2 }
    };
    (je, $label:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Je {
            label: $label.to_string(),
        }
    };
    (jne, $label:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Jne {
            label: $label.to_string(),
        }
    };
    (jg, $label:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Jg {
            label: $label.to_string(),
        }
    };
    (jl, $label:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Jl {
            label: $label.to_string(),
        }
    };
    (jge, $label:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Jge {
            label: $label.to_string(),
        }
    };
    (jle, $label:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Jle {
            label: $label.to_string(),
        }
    };
    (wait, $time:expr) => {
        logix_sim::primitives::prim_program::PrimCommand::Wait { time: $time }
    };
}

impl PrimProgram {
    pub fn new(update_type: ProgramUpdateType, cmds: Vec<PrimCommand>) -> Self {
        let mut label_pos = HashMap::new();
        for (i, cmd) in cmds.iter().enumerate() {
            if let PrimCommand::Label { name } = cmd {
                label_pos.insert(name.clone(), i);
            }
        }
        Self {
            cmds,
            update_type,
            state: PrimProgramState {
                vars: HashMap::new(),
                pc: 0,
                cmp_flag: 0,
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

    pub fn eval_expr(&mut self, expr: &PrimExpr) -> Data {
        match expr {
            PrimExpr::Not(expr) => !self.eval_expr(expr),
            PrimExpr::And(exprs) => exprs
                .iter()
                .fold(Data::high(), |acc, x| acc & self.eval_expr(x)),
            PrimExpr::Or(exprs) => exprs
                .iter()
                .fold(Data::low(), |acc, x| acc | self.eval_expr(x)),
            PrimExpr::Nand(exprs) => exprs
                .iter()
                .fold(Data::high(), |acc, x| acc & self.eval_expr(x)),
            PrimExpr::Nor(exprs) => exprs
                .iter()
                .fold(Data::low(), |acc, x| acc | self.eval_expr(x)),
            PrimExpr::Xor(exprs) => exprs
                .iter()
                .skip(1)
                .fold(self.eval_expr(&exprs[0]), |acc, x| acc ^ self.eval_expr(x)),
            PrimExpr::Var(name) => self.state.vars[name],
            PrimExpr::Const(value) => *value,
            PrimExpr::BitVec(exprs) => {
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
                PrimCommand::Mov { name, value } => {
                    let val = self.eval_expr(&value);
                    self.state.vars.insert(name, val);
                    self.state.pc += 1;
                }
                PrimCommand::Label { .. } => {
                    self.state.pc += 1;
                }
                PrimCommand::Goto { label } => {
                    let pos = self.state.label_pos[&label];
                    self.state.pc = pos;
                }
                PrimCommand::CmpVar { name, value } => {
                    let var_val = self.state.vars[&name];
                    let val = self.eval_expr(&value);
                    self.state.cmp_flag = if var_val.value < val.value {
                        -1
                    } else if var_val == val {
                        0
                    } else {
                        1
                    };
                    self.state.pc += 1;
                }
                PrimCommand::Cmp { v1, v2 } => {
                    let v1 = self.eval_expr(&v1);
                    let v2 = self.eval_expr(&v2);
                    self.state.cmp_flag = if v1.value < v2.value {
                        -1
                    } else if v1 == v2 {
                        0
                    } else {
                        1
                    };
                    self.state.pc += 1;
                }
                PrimCommand::Je { label } => {
                    if self.state.cmp_flag == 0 {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PrimCommand::Jne { label } => {
                    if self.state.cmp_flag != 0 {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PrimCommand::Jg { label } => {
                    if self.state.cmp_flag == 1 {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PrimCommand::Jl { label } => {
                    if self.state.cmp_flag == -1 {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PrimCommand::Jge { label } => {
                    if self.state.cmp_flag == 1 || self.state.cmp_flag == 0 {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PrimCommand::Jle { label } => {
                    if self.state.cmp_flag == -1 || self.state.cmp_flag == 0 {
                        let pos = self.state.label_pos[&label];
                        self.state.pc = pos;
                    } else {
                        self.state.pc += 1;
                    }
                }
                PrimCommand::Wait { time } => {
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
