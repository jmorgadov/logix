use log::debug;

use crate::{
    parser::grammar_mod_builder::grammar::CommandParser,
    pcmd, pexp,
    program::{AsmCommand, AsmExpr, AsmProgramUpdateType},
    AsmComponent,
};

#[derive(Debug)]
pub enum AsmCommandArg {
    Var(String),
    Num(usize),
    Expr(AsmExpr),
}

pub enum AsmCmdDecl {
    Mov,
    Label,
    Goto,
    Cmp,
    Jmp,
    Je,
    Jne,
    Jg,
    Jge,
    Jl,
    Jle,
    Wait,
}

enum ParseState {
    Info,
    Inputs,
    Outputs,
    Defaults,
    Commands,
}

impl AsmCommandArg {
    pub fn get_expr(&self) -> AsmExpr {
        match self {
            AsmCommandArg::Expr(expr) => expr.clone(),
            AsmCommandArg::Var(name) => pexp!(var, name.clone()),
            AsmCommandArg::Num(num) => pexp!(val, *num),
        }
    }
}

impl AsmComponent {
    fn remove_comments(line: &str) -> String {
        let mut line = line.to_string();
        if let Some(pos) = line.find("//") {
            line.truncate(pos);
        }
        line.trim().to_string()
    }

    pub fn parse(code: &str) -> AsmComponent {
        let mut state = ParseState::Info;
        let mut ast = AsmComponent::default();

        for line in code.split("\n") {
            let line = Self::remove_comments(line);

            if line.is_empty() {
                continue;
            }

            debug!("Parsing line: {}", line);

            if line.starts_with("_info:") {
                state = ParseState::Info;
                continue;
            } else if line.starts_with("_inputs:") {
                state = ParseState::Inputs;
                continue;
            } else if line.starts_with("_outputs:") {
                state = ParseState::Outputs;
                continue;
            } else if line.starts_with("_defaults:") {
                state = ParseState::Defaults;
                continue;
            } else if line.starts_with("_start:") {
                state = ParseState::Commands;
                continue;
            }

            match state {
                ParseState::Info => {
                    let info_cmd = line
                        .split(" ")
                        .take(1)
                        .next()
                        .expect("Expected info command");
                    if info_cmd == "name" {
                        ast.info.name = line.split(" ").skip(1).collect();
                    } else if info_cmd == "description" {
                        ast.info.description = Some(
                            line.split(" ")
                                .skip(1)
                                .fold(String::new(), |acc, x| acc + " " + x)
                                .trim()
                                .to_string(),
                        );
                    } else if info_cmd == "update_type" {
                        let update_type: String = line
                            .split(" ")
                            .skip(1)
                            .fold(String::new(), |acc, x| acc + " " + x)
                            .trim()
                            .to_string();
                        if update_type == "input_changes" {
                            ast.info.update_type = AsmProgramUpdateType::InputChanges;
                        } else if update_type == "always" {
                            ast.info.update_type = AsmProgramUpdateType::Always;
                        } else {
                            panic!("Invalid update type");
                        }
                    }
                }
                ParseState::Inputs => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    ast.inputs
                        .insert(parts[0].to_string(), parts[1].parse().unwrap());
                }
                ParseState::Outputs => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    ast.outputs
                        .insert(parts[0].to_string(), parts[1].parse().unwrap());
                }
                ParseState::Defaults => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let name = parts[0].to_string();
                    let value = parts[1].into();
                    ast.defaults.insert(name, value);
                }
                ParseState::Commands => {
                    ast.cmds
                        .push(CommandParser::new().parse(&line).expect("Invalid command"));
                }
            }
        }

        ast
    }
}

pub fn cmd_from_args(cmd: AsmCmdDecl, args: Vec<AsmCommandArg>) -> AsmCommand {
    match cmd {
        AsmCmdDecl::Mov => {
            let name = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            let expr = match args.get(1) {
                Some(arg) => arg.get_expr(),
                _ => panic!("Expected argument expr"),
            };
            pcmd!(mov, name, expr)
        }
        AsmCmdDecl::Label => {
            let name = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(label, name)
        }
        AsmCmdDecl::Goto => {
            let name = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(goto, name)
        }
        AsmCmdDecl::Cmp => {
            let v1 = match args.first() {
                Some(arg) => arg.get_expr(),
                _ => panic!("Expected argument expr"),
            };
            let v2 = match args.get(1) {
                Some(arg) => arg.get_expr(),
                _ => panic!("Expected argument expr"),
            };
            pcmd!(cmp, v1, v2)
        }
        AsmCmdDecl::Jmp => {
            let label = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(jmp, label)
        }
        AsmCmdDecl::Je => {
            let label = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(je, label)
        }
        AsmCmdDecl::Jne => {
            let label = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(jne, label)
        }
        AsmCmdDecl::Jg => {
            let label = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(jg, label)
        }
        AsmCmdDecl::Jge => {
            let label = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(jge, label)
        }
        AsmCmdDecl::Jl => {
            let label = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(jl, label)
        }
        AsmCmdDecl::Jle => {
            let label = match args.first() {
                Some(AsmCommandArg::Var(name)) => name,
                _ => panic!("Invalid argument"),
            };
            pcmd!(jle, label)
        }
        AsmCmdDecl::Wait => {
            let time = match args.first() {
                Some(AsmCommandArg::Num(time)) => time,
                _ => panic!("Invalid argument"),
            };
            pcmd!(wait, *time as u128)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::program::AsmCommand;

    use super::*;
    #[test]
    fn test_cmd_cover() {
        let pasm_cmd = AsmCommand::Label {
            name: "test".to_string(),
        };
        let _ = match pasm_cmd {
            AsmCommand::Mov { .. } => AsmCmdDecl::Mov,
            AsmCommand::Label { .. } => AsmCmdDecl::Label,
            AsmCommand::Goto { .. } => AsmCmdDecl::Goto,
            AsmCommand::Cmp { .. } => AsmCmdDecl::Cmp,
            AsmCommand::Jmp { .. } => AsmCmdDecl::Jmp,
            AsmCommand::Je { .. } => AsmCmdDecl::Je,
            AsmCommand::Jne { .. } => AsmCmdDecl::Jne,
            AsmCommand::Jg { .. } => AsmCmdDecl::Jg,
            AsmCommand::Jl { .. } => AsmCmdDecl::Jl,
            AsmCommand::Jge { .. } => AsmCmdDecl::Jge,
            AsmCommand::Jle { .. } => AsmCmdDecl::Jle,
            AsmCommand::Wait { .. } => AsmCmdDecl::Wait,
        };
    }
}
