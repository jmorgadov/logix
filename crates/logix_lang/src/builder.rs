use lalrpop_util::lalrpop_mod;
use log::debug;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

use logix_core::component::{Component, ComponentBuilder, Conn, PortAddr};
use logix_sim::primitives::{
    primitive::ExtraInfo,
    primitive_builders::{
        and_gate, clock, high_const, input, joiner, low_const, nand_gate, nor_gate, not_gate,
        or_gate, output, splitter, xor_gate,
    },
};

use crate::ast::prelude::*;

lalrpop_mod!(pub grammar);

#[derive(Debug, Clone, Error)]
pub enum BuildError {
    #[error("No main component found")]
    NoMainComponentError,

    #[error("Component declaration not found: {0}")]
    ComponentDeclNotFoundError(String),

    #[error("Component reference not found: {0}")]
    ComponentRefNotFoundError(String),

    #[error("Input pin not found: {0}")]
    InputPinNotFoundError(String),

    #[error("Output pin not found: {0}")]
    OutputPinNotFoundError(String),

    #[error("Subcircuit module not found: {0}")]
    ImportError(String),

    #[error("[{0}] Syntax error: {1}")]
    ModuleSintaxError(String, String),
}

pub fn build_from_file(
    main_path: &str,
) -> Result<(Component<ExtraInfo>, HashMap<usize, String>), BuildError> {
    debug!("Building from file: {}", main_path);
    let comp_map = get_comp_map(main_path.to_string())?;
    let main = comp_map
        .get("Main")
        .ok_or(BuildError::NoMainComponentError)?;
    let mut last_id: usize = 0;
    let mut id_map: HashMap<usize, String> = HashMap::new();
    let comp = comp_decl_to_comp(main, "main", &comp_map, &mut last_id, &mut id_map)?;
    Ok((comp, id_map))
}

fn get_loc(loc: usize, text: &str) -> (usize, usize) {
    let mut line = 1;
    let mut col = 0;
    for (i, c) in text.chars().enumerate() {
        if i == loc {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn get_comp_map(lgx_path: String) -> Result<HashMap<String, Box<CompDecl>>, BuildError> {
    debug!("Getting component map from: {}", lgx_path);

    let text = std::fs::read_to_string(lgx_path.to_string())
        .map_err(|_| BuildError::ImportError(lgx_path.to_string()))?;

    debug!("Parsing file: {}", lgx_path);
    let circuit = grammar::CircuitParser::new().parse(&text).map_err(|e| {
        let (line, col) = match &e {
            lalrpop_util::ParseError::InvalidToken { location } => get_loc(*location, &text),
            lalrpop_util::ParseError::UnrecognizedEof {
                location,
                expected: _,
            } => get_loc(*location, &text),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected: _ } => {
                get_loc(token.0, &text)
            }
            lalrpop_util::ParseError::ExtraToken { token: _ } => todo!(),
            lalrpop_util::ParseError::User { error: _ } => todo!(),
        };
        BuildError::ModuleSintaxError(
            lgx_path.to_string(),
            format!("[{}:{}] {}", line, col, e.to_string()),
        )
    })?;

    debug!("Building component map");
    let mut comp_map: HashMap<String, Box<CompDecl>> = circuit
        .comps
        .into_iter()
        .map(|comp| (comp.name.clone(), Box::new(comp)))
        .collect();

    debug!(
        "File comp_map: {:?}",
        comp_map.keys().collect::<Vec<&String>>()
    );

    if let Some(imports) = &circuit.imports {
        debug!("Processing imports: {:?}", imports);
        for import in imports {
            let path = Path::new(&lgx_path)
                .parent()
                .unwrap()
                .join(format!("{}.lgx", import))
                .to_str()
                .unwrap()
                .to_string();
            debug!("Importing: {}", path);
            let imported_map = get_comp_map(path)?;
            comp_map.extend(imported_map);
        }
    }

    Ok(comp_map)
}

fn comp_decl_to_comp(
    comp: &CompDecl,
    name: &str,
    comp_map: &HashMap<String, Box<CompDecl>>,
    last_id: &mut usize,
    id_map: &mut HashMap<usize, String>,
) -> Result<Component<ExtraInfo>, BuildError> {
    debug!("Processing component: {}", name);

    let mut subc = comp.subc.iter().collect::<Vec<_>>();

    subc.sort_by(|a, b| a.0 .1.cmp(&b.0 .1));

    let subc_map = subc
        .iter()
        .enumerate()
        .map(|(i, ((n, _), _))| (n.clone(), i))
        .collect();

    let subc: Vec<Component<ExtraInfo>> = subc
        .iter()
        .map(|((subc_name, _), sub_comp)| {
            let sub_c = match sub_comp {
                Comp::Primitive(prim) => {
                    *last_id += 1;
                    let prim = match prim {
                        Primitive::And(ins_count) => and_gate(*last_id, *ins_count),
                        Primitive::Or(ins_count) => or_gate(*last_id, *ins_count),
                        Primitive::Not => not_gate(*last_id),
                        Primitive::Nand(ins_count) => nand_gate(*last_id, *ins_count),
                        Primitive::Nor(ins_count) => nor_gate(*last_id, *ins_count),
                        Primitive::HighConst => high_const(*last_id),
                        Primitive::LowConst => low_const(*last_id),
                        Primitive::Clock(frec) => clock(*last_id, *frec),
                        Primitive::Xor(ins_count) => xor_gate(*last_id, *ins_count),
                        Primitive::Input(bits) => input(*last_id, *bits),
                        Primitive::Output(bits) => output(*last_id, *bits),
                        Primitive::Splitter(bits) => splitter(*last_id, *bits),
                        Primitive::Joiner(bits) => joiner(*last_id, *bits),
                    };
                    id_map.insert(*last_id, subc_name.to_string());
                    debug!("Creating primitive: {} with id {}", subc_name, *last_id);
                    Ok(prim)
                }
                Comp::Composite(name) => {
                    let decl = comp_map
                        .get(name)
                        .ok_or(BuildError::ComponentDeclNotFoundError(name.to_string()))?;
                    let compose = comp_decl_to_comp(decl, subc_name, comp_map, last_id, id_map)?;
                    Ok(compose)
                }
            };
            sub_c
        })
        .collect::<Result<Vec<Component<ExtraInfo>>, BuildError>>()?;

    let conns: Vec<Conn> = get_connections(comp, &subc_map, comp_map)?;

    let mut in_addrs: Vec<(usize, PortAddr)> = vec![];
    let mut out_addrs: Vec<PortAddr> = vec![];

    for (i, idx) in comp.ins.iter().enumerate() {
        in_addrs.push((i, (*idx, 0)));
    }

    for idx in comp.outs.iter() {
        out_addrs.push((*idx, 0));
    }

    let in_count: usize = in_addrs.len();
    let out_count: usize = out_addrs.len();

    *last_id += 1;
    id_map.insert(*last_id, name.to_string());
    debug!("Creating component: {} with id {}", name, *last_id);
    debug!("{} In addrs: {:?}", in_count, in_addrs);
    debug!("{} Out addrs: {:?}", out_count, out_addrs);

    Ok(ComponentBuilder::new(*last_id)
        .name(name.to_string())
        .port_count(in_count, out_count)
        .sub_comps(subc)
        .connections(conns)
        .in_addrs(in_addrs)
        .out_addrs(out_addrs)
        .extra(ExtraInfo::new(*last_id))
        .build())
}

fn get_connections(
    comp: &CompDecl,
    subc_map: &HashMap<String, usize>,
    comp_map: &HashMap<String, Box<CompDecl>>,
) -> Result<Vec<Conn>, BuildError> {
    debug!("Processing connections for: {}", comp.name);

    let mut conns = vec![];

    let get_subc_idx = |name: &str| -> Result<usize, BuildError> {
        subc_map
            .get(name)
            .ok_or(BuildError::ComponentRefNotFoundError(name.to_string()))
            .map(|x| *x)
    };

    let get_pin_idx =
        |comp_name: &str, pin_name: &str, is_input: bool| -> Result<usize, BuildError> {
            let comp = get_comp_decl(comp, comp_name, comp_map)
                .map_err(|_| BuildError::ComponentRefNotFoundError(comp_name.to_string()))?;
            if is_input {
                comp.out_idx_by_name
                    .get(pin_name)
                    .map(|x| *x)
                    .ok_or(BuildError::InputPinNotFoundError(pin_name.to_string()))
            } else {
                comp.in_idx_by_name
                    .get(pin_name)
                    .map(|x| *x)
                    .ok_or(BuildError::OutputPinNotFoundError(pin_name.to_string()))
            }
        };

    for conn in &comp.design {
        debug!("|  Processing connection: {:?}", conn);

        let src_comp_idx = get_subc_idx(&conn.src.name())?;
        let dest_comp_idx = get_subc_idx(&conn.dest.name())?;

        let src_pin_idx = match &conn.src {
            PinAddr::ByName(_, n) => get_pin_idx(&conn.src.name(), n, true)?,
            PinAddr::ByIdx(_, idx) => *idx,
        };

        let dest_pin_idx = match &conn.dest {
            PinAddr::ByName(_, n) => get_pin_idx(&conn.dest.name(), n, false)?,
            PinAddr::ByIdx(_, idx) => *idx,
        };

        conns.push(Conn::new(
            src_comp_idx,
            src_pin_idx,
            dest_comp_idx,
            dest_pin_idx,
        ));

        debug!(
            "|  Internal to internal: (({}, {}), ({}, {}))",
            src_comp_idx, src_pin_idx, dest_comp_idx, dest_pin_idx
        );
    }

    Ok(conns)
}

fn get_comp_decl<'a>(
    comp: &'a CompDecl,
    name: &'a str,
    comp_map: &'a HashMap<String, Box<CompDecl>>,
) -> Result<&'a CompDecl, BuildError> {
    let subc = comp
        .subc
        .iter()
        .filter_map(|((n, _), c)| if n == name { Some(c) } else { None })
        .next()
        .ok_or(BuildError::ComponentRefNotFoundError(name.to_string()))?;

    let subc_type = match subc {
        Comp::Primitive(prim) => prim.to_string(),
        Comp::Composite(name) => name.clone(),
    };
    let comp_decl = comp_map
        .get(&subc_type)
        .ok_or(BuildError::ComponentDeclNotFoundError(subc_type.clone()))?;

    Ok(comp_decl)
}
