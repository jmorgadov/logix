use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::value::AsmValue;

const FLAG_EQUAL: usize = 0;
const FLAG_LESS: usize = 1;

/// Update type of the program
#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub enum AsmProgramUpdateType {
    /// Only update when the input values change
    #[default]
    InputChanges,

    /// Update constantly
    ///
    /// Good for defining a clock signal
    Always,
}

/// Represents a running state of an AsmHDL program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsmProgramState {
    /// Commands
    pub cmds: Vec<AsmCommand>,

    /// Declared variables
    pub vars: HashMap<String, AsmValue>,

    /// Program counter
    pub pc: usize,

    /// Flags
    ///  bit\val   0        1
    ///    0       equal    not equal
    ///    1       greater  less
    pub flags: usize,

    /// Line position of labels
    pub label_pos: HashMap<String, usize>,

    /// Time to wait from
    pub waiting_from: Option<u128>,
}

/// AsmHDL expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsmExpr {
    /// Logical not
    Not(Box<AsmExpr>),
    /// Logical and
    And(Vec<AsmExpr>),
    /// Logical or
    Or(Vec<AsmExpr>),
    /// Logical nand
    Nand(Vec<AsmExpr>),
    /// Logical nor
    Nor(Vec<AsmExpr>),
    /// Logical xor
    Xor(Vec<AsmExpr>),
    /// Concatenation of bits
    BitVec(Vec<AsmExpr>),
    /// variable
    ///
    /// From the program state
    Var(String),
    /// Constant value
    Const(AsmValue),
}

/// AsmHDL command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsmCommand {
    /// Sets a value to a variable
    Mov {
        /// Variable name
        name: String,
        /// Value to set
        value: AsmExpr,
    },
    /// Declares a label
    Label {
        /// Label name
        name: String,
    },
    /// Goto a label
    Goto {
        /// Label to go to
        label: String,
    },
    /// Compares two values (this sets the flags)
    Cmp {
        /// First value
        v1: AsmExpr,
        /// Second value
        v2: AsmExpr,
    },
    /// Jumps to a label if the cmp flags was 'equal'
    Je {
        /// Label to jump to
        label: String,
    },
    /// Jumps to a label if the cmp flags was 'not equal'
    Jne {
        /// Label to jump to
        label: String,
    },

    /// Jumps to a label if the cmp flags was 'greater than'
    Jg {
        /// Label to jump to
        label: String,
    },

    /// Jumps to a label if the cmp flags was 'less than'
    Jl {
        /// Label to jump to
        label: String,
    },
    /// Jumps to a label if the cmp flags was 'greater than or equal'
    Jge {
        /// Label to jump to
        label: String,
    },
    /// Jumps to a label if the cmp flags was 'less than or equal'
    Jle {
        /// Label to jump to
        label: String,
    },
    /// Waits for a certain amount of time (ns)
    Wait {
        /// Time to wait
        time: u128,
    },
}

/// Macro to create an AsmExpr
#[macro_export]
macro_rules! pexp {
    (val, $val:expr) => {
        $crate::AsmExpr::Const($val.into())
    };
    (var, $name:expr) => {
        $crate::AsmExpr::Var($name.to_string())
    };
    (not, $expr:expr) => {
        $crate::AsmExpr::Not(Box::new($expr))
    };
    (and, $($expr:expr),*) => {
        $crate::AsmExpr::And(vec![$($expr),*])
    };
    (or, $($expr:expr),*) => {
        $crate::AsmExpr::Or(vec![$($expr),*])
    };
    (nand, $($expr:expr),*) => {
        $crate::AsmExpr::Nand(vec![$($expr),*])
    };
    (nor, $($expr:expr),*) => {
        $crate::AsmExpr::Nor(vec![$($expr),*])
    };
    (xor, $($expr:expr),*) => {
        $crate::AsmExpr::Xor(vec![$($expr),*])
    };
    (bit_vec, $($expr:expr),*) => {
        $crate::AsmExpr::BitVec(vec![$($expr),*])
    };
    (bit_vecv, $exprs:expr) => {
        $crate::AsmExpr::BitVec($exprs)
    };
}

/// Macro to create an AsmCommand
#[macro_export]
macro_rules! pcmd {
    (mov, $name:expr, $val:expr) => {
        $crate::AsmCommand::Mov {
            name: $name.to_string(),
            value: $val,
        }
    };
    (label, $name:expr) => {
        $crate::AsmCommand::Label {
            name: $name.to_string(),
        }
    };
    (goto, $label:expr) => {
        $crate::AsmCommand::Goto {
            label: $label.to_string(),
        }
    };
    (cmp, $v1:expr, $v2:expr) => {
        $crate::AsmCommand::Cmp { v1: $v1, v2: $v2 }
    };
    (je, $label:expr) => {
        $crate::AsmCommand::Je {
            label: $label.to_string(),
        }
    };
    (jne, $label:expr) => {
        $crate::AsmCommand::Jne {
            label: $label.to_string(),
        }
    };
    (jg, $label:expr) => {
        $crate::AsmCommand::Jg {
            label: $label.to_string(),
        }
    };
    (jl, $label:expr) => {
        $crate::AsmCommand::Jl {
            label: $label.to_string(),
        }
    };
    (jge, $label:expr) => {
        $crate::AsmCommand::Jge {
            label: $label.to_string(),
        }
    };
    (jle, $label:expr) => {
        $crate::AsmCommand::Jle {
            label: $label.to_string(),
        }
    };
    (wait, $time:expr) => {
        $crate::AsmCommand::Wait { time: $time }
    };
}

impl AsmProgramState {
    /// Creates a new program state with the given update type and commands
    pub fn new(cmds: Vec<AsmCommand>) -> Self {
        let mut label_pos = HashMap::new();
        for (i, cmd) in cmds.iter().enumerate() {
            if let AsmCommand::Label { name } = cmd {
                label_pos.insert(name.clone(), i);
            }
        }
        AsmProgramState {
            cmds,
            vars: HashMap::new(),
            pc: 0,
            flags: 0,
            label_pos,
            waiting_from: None,
        }
    }

    /// Sets the default variables of the program
    pub fn with_default_vars(
        mut self,
        vars: HashMap<impl Into<String>, impl Into<AsmValue>>,
    ) -> Self {
        self.vars = vars
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    fn set_flag(&mut self, bit: usize, val: bool) {
        if val {
            self.flags |= 1 << bit;
        } else {
            self.flags &= !(1 << bit);
        }
    }

    fn flag_at(&self, bit: usize) -> bool {
        (self.flags & (1 << bit)) != 0
    }

    fn eval_expr(&mut self, expr: &AsmExpr) -> AsmValue {
        match expr {
            AsmExpr::Not(expr) => !self.eval_expr(expr),
            AsmExpr::And(exprs) => exprs
                .iter()
                .fold(AsmValue::true_val(), |acc, x| acc & self.eval_expr(x)),
            AsmExpr::Or(exprs) => exprs
                .iter()
                .fold(AsmValue::false_val(), |acc, x| acc | self.eval_expr(x)),
            AsmExpr::Nand(exprs) => !exprs
                .iter()
                .fold(AsmValue::true_val(), |acc, x| acc & self.eval_expr(x)),
            AsmExpr::Nor(exprs) => !exprs
                .iter()
                .fold(AsmValue::false_val(), |acc, x| acc | self.eval_expr(x)),
            AsmExpr::Xor(exprs) => exprs
                .iter()
                .skip(1)
                .fold(self.eval_expr(&exprs[0]), |acc, x| acc ^ self.eval_expr(x)),
            AsmExpr::Var(name) => self.vars[name],
            AsmExpr::Const(value) => *value,
            AsmExpr::BitVec(exprs) => {
                let mut data = AsmValue::new(0, exprs.len());
                for (i, expr) in exprs.iter().rev().enumerate() {
                    data.set_bit(i, self.eval_expr(expr).as_bool());
                }
                data
            }
        }
    }

    /// Runs the program
    ///
    /// This will execute the commands of the program until it finishes or it waits for some
    /// time.
    pub fn run(&mut self, curr_time: u128) {
        let mut running = true;
        while running {
            let pc = self.pc;

            match self.cmds[pc].clone() {
                AsmCommand::Mov { name, value } => {
                    let val = self.eval_expr(&value);
                    self.vars.insert(name, val);
                    self.pc += 1;
                }
                AsmCommand::Label { .. } => {
                    self.pc += 1;
                }
                AsmCommand::Goto { label } => {
                    let pos = self.label_pos[&label];
                    self.pc = pos;
                }
                AsmCommand::Cmp { v1, v2 } => {
                    let v1 = self.eval_expr(&v1);
                    let v2 = self.eval_expr(&v2);
                    self.set_flag(FLAG_EQUAL, v1.value == v2.value);
                    self.set_flag(FLAG_LESS, v1.value < v2.value);
                    self.pc += 1;
                }
                AsmCommand::Je { label } => {
                    if self.flag_at(FLAG_EQUAL) {
                        let pos = self.label_pos[&label];
                        self.pc = pos;
                    } else {
                        self.pc += 1;
                    }
                }
                AsmCommand::Jne { label } => {
                    if !self.flag_at(FLAG_EQUAL) {
                        let pos = self.label_pos[&label];
                        self.pc = pos;
                    } else {
                        self.pc += 1;
                    }
                }
                AsmCommand::Jg { label } => {
                    if !self.flag_at(FLAG_LESS) {
                        let pos = self.label_pos[&label];
                        self.pc = pos;
                    } else {
                        self.pc += 1;
                    }
                }
                AsmCommand::Jl { label } => {
                    if self.flag_at(FLAG_LESS) {
                        let pos = self.label_pos[&label];
                        self.pc = pos;
                    } else {
                        self.pc += 1;
                    }
                }
                AsmCommand::Jge { label } => {
                    if self.flag_at(FLAG_EQUAL) || !self.flag_at(FLAG_LESS) {
                        let pos = self.label_pos[&label];
                        self.pc = pos;
                    } else {
                        self.pc += 1;
                    }
                }
                AsmCommand::Jle { label } => {
                    if self.flag_at(FLAG_EQUAL) || self.flag_at(FLAG_LESS) {
                        let pos = self.label_pos[&label];
                        self.pc = pos;
                    } else {
                        self.pc += 1;
                    }
                }
                AsmCommand::Wait { time } => {
                    if let Some(from) = self.waiting_from {
                        if from + time <= curr_time {
                            self.waiting_from = None;
                            self.pc += 1;
                        } else {
                            running = false;
                        }
                    } else {
                        self.waiting_from = Some(curr_time);
                        running = false;
                    }
                }
            }

            if pc >= self.cmds.len() - 1 {
                // Just start again in the next update (run)
                self.pc = 0;
                running = false;
            }
        }
    }
}
