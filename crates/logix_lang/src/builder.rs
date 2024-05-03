use lalrpop_util::lalrpop_mod;
use log::debug;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

use logix_core::component::{Component, ComponentBuilder, Conn, PortAddr};
use logix_sim::primitives::{
    primitive_builders::{
        and_gate, clock, high_const, low_const, nand_gate, not_gate, or_gate, xor_gate,
    },
    primitives::ExtraInfo,
};

use crate::ast::{prelude::*, PinIndexing};

lalrpop_mod!(pub grammar);

#[derive(Debug, Clone, Error)]
pub enum BuildError {
    #[error("No main component found")]
    NoMainComponentError,

    #[error("External to external connection detected")]
    ExternalToExternalConnectionError,

    #[error("Component declaration not found: {0}")]
    ComponentDeclNotFoundError(String),

    #[error("Component reference not found: {0}")]
    ComponentRefNotFoundError(String),

    #[error("Input pin not found: {0}")]
    InputPinNotFoundError(String),

    #[error("Output pin not found: {0}")]
    OutputPinNotFoundError(String),

    #[error("Internal pin not found: ({0}, {1})")]
    InternalPinError(String, usize),

    #[error("Invalid pin range")]
    InvalidPinRange,

    #[error("Invalid range connection")]
    InvalidRangeConection,

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
    let subc_map: HashMap<String, usize> = comp
        .subc
        .iter()
        .enumerate()
        .map(|(idx, (name, _))| (name.clone(), idx))
        .collect();

    let subc: Vec<Component<ExtraInfo>> = comp
        .subc
        .iter()
        .map(|(subc_name, sub_comp)| {
            let sub_c = match sub_comp {
                Comp::Primitive(prim) => {
                    *last_id += 1;
                    let prim = match prim {
                        Primitive::And(ins_count) => and_gate(*last_id, *ins_count),
                        Primitive::Or(ins_count) => or_gate(*last_id, *ins_count),
                        Primitive::Not => not_gate(*last_id),
                        Primitive::Nand(ins_count) => nand_gate(*last_id, *ins_count),
                        Primitive::Nor(ins_count) => nand_gate(*last_id, *ins_count),
                        Primitive::HighConst => high_const(*last_id),
                        Primitive::LowConst => low_const(*last_id),
                        Primitive::Clock(frec) => clock(*last_id, *frec),
                        Primitive::Xor(ins_count) => xor_gate(*last_id, *ins_count),
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

    let (in_addrs, out_addrs, conns) = get_connections(comp, &subc, &subc_map, comp_map)?;

    let in_count: usize = comp.ins.values().map(|x| x.1).sum::<u8>() as usize;
    let out_count: usize = comp.outs.values().map(|x| x.1).sum::<u8>() as usize;

    *last_id += 1;
    id_map.insert(*last_id, name.to_string());
    debug!("Creating component: {} with id {}", name, *last_id);
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
    subc: &Vec<Component<ExtraInfo>>,
    subc_map: &HashMap<String, usize>,
    comp_map: &HashMap<String, Box<CompDecl>>,
) -> Result<(Vec<(usize, PortAddr)>, Vec<PortAddr>, Vec<Conn>), BuildError> {
    debug!("Processing connections for: {}", comp.name);

    let mut in_addrs = vec![];
    let mut out_addrs = vec![(0, 0); comp.outs.values().map(|x| x.1).sum::<u8>() as usize];
    let mut conns = vec![];

    let get_subc_idx = |name: &str| -> Result<usize, BuildError> {
        subc_map
            .get(name)
            .ok_or(BuildError::ComponentRefNotFoundError(name.to_string()))
            .map(|x| *x)
    };

    for conn in &comp.design {
        debug!("|  Processing connection: {:?}", conn);

        let src = internal_name_to_idx(&conn.src, comp, comp_map, false)?;
        let dest = internal_name_to_idx(&conn.dest, comp, comp_map, true)?;

        debug!("|  Resolved connection: ({:?}, {:?})", src, dest);

        let (src_bit_idx, dest_bit_idx, len) = preprocess_indexing_range(&src, &dest)?;

        debug!(
            "|  Preprocessed indexing range: ({}, {}, {})",
            src_bit_idx, dest_bit_idx, len
        );

        match (src, dest) {
            //
            // From input pin to internal component
            (PinAddr::External(in_name, _), PinAddr::InternalIdx(dest_name, idx, _)) => {
                debug!(
                    "|  Input pin to internal: ({}, ({}, {}))",
                    in_name, dest_name, idx
                );

                let in_idx = comp
                    .ins
                    .get(&in_name)
                    .ok_or(BuildError::InputPinNotFoundError(in_name.clone()))?
                    .0;

                for i in 0..len {
                    // Check if the idx is in range
                    let subc_idx = get_subc_idx(&dest_name)?;
                    if subc[subc_idx].inputs <= idx + dest_bit_idx + i {
                        return Err(BuildError::InternalPinError(
                            dest_name.clone(),
                            idx + dest_bit_idx + i,
                        ));
                    }
                    in_addrs.push((in_idx + src_bit_idx + i, (subc_idx, idx + dest_bit_idx + i)));
                    debug!("|    In addr created: {:?}", in_addrs.last().unwrap());
                }
            }
            //
            // From internal component to output pin
            (PinAddr::InternalIdx(src_name, idx, _), PinAddr::External(out_name, _)) => {
                debug!(
                    "|  Internal to output pin: (({}, {}), {})",
                    src_name, idx, out_name
                );
                let out_idx = comp
                    .outs
                    .get(&out_name)
                    .ok_or(BuildError::OutputPinNotFoundError(out_name.clone()))?
                    .0;

                let subc_idx = get_subc_idx(&src_name)?;

                for i in 0..len {
                    // Check if the idx is in range
                    if subc[subc_idx].outputs <= idx + src_bit_idx + i {
                        return Err(BuildError::InternalPinError(
                            src_name.clone(),
                            idx + src_bit_idx + i,
                        ));
                    }
                    out_addrs[out_idx + dest_bit_idx + i] = (subc_idx, idx + src_bit_idx + i);
                    debug!(
                        "|    Out addr created: {:?} at {}",
                        out_addrs[out_idx + i],
                        out_idx + dest_bit_idx + i
                    );
                }
            }
            //
            // From internal to internal component
            (
                PinAddr::InternalIdx(src_name, src_idx, _),
                PinAddr::InternalIdx(dest_name, dest_idx, _),
            ) => {
                for i in 0..len {
                    conns.push(Conn::new(
                        get_subc_idx(&src_name)?,
                        src_idx + src_bit_idx + i,
                        get_subc_idx(&dest_name)?,
                        dest_idx + dest_bit_idx + i,
                    ));
                    debug!(
                        "|  Internal to internal: (({}, {}), ({}, {}))",
                        src_name,
                        src_idx + src_bit_idx + i,
                        dest_name,
                        dest_idx + dest_bit_idx + i
                    );
                    debug!("|    Connection created: {:?}", conns.last().unwrap())
                }
            }
            (PinAddr::External(_, _), PinAddr::External(_, _)) => {
                return Err(BuildError::ExternalToExternalConnectionError);
            }
            _ => unreachable!(),
        }
    }

    Ok((in_addrs, out_addrs, conns))
}

fn preprocess_indexing_range(
    src: &PinAddr,
    dest: &PinAddr,
) -> Result<(usize, usize, usize), BuildError> {
    let src_indexing = match src {
        PinAddr::InternalIdx(_, _, idx) => idx,
        PinAddr::External(_, idx) => idx,
        _ => unreachable!(),
    };
    let dest_indexing = match dest {
        PinAddr::InternalIdx(_, _, idx) => idx,
        PinAddr::External(_, idx) => idx,
        _ => unreachable!(),
    };
    if let (PinIndexing::Range(si, sj), PinIndexing::Range(di, dj)) =
        (src_indexing.clone(), dest_indexing.clone())
    {
        if si > sj || di > dj {
            return Err(BuildError::InvalidPinRange);
        }
    }
    let (src_idx, dest_idx, len): (u8, u8, u8) = match (*src_indexing, *dest_indexing) {
        (PinIndexing::Range(si, sj), PinIndexing::Range(di, dj)) => {
            if sj - si != dj - di {
                return Err(BuildError::InvalidRangeConection);
            }
            (si, di, (sj - si) + 1)
        }
        (PinIndexing::Range(si, sj), PinIndexing::Index(d)) => {
            if sj != sj {
                return Err(BuildError::InvalidRangeConection);
            }
            (si, d, 1)
        }
        (PinIndexing::Range(si, sj), PinIndexing::NoIndex) => {
            if si != sj {
                return Err(BuildError::InvalidRangeConection);
            }
            (si, 0, 1)
        }
        (PinIndexing::NoIndex, PinIndexing::NoIndex) => (0, 0, 1),
        (PinIndexing::NoIndex, PinIndexing::Index(d)) => (0, d, 1),
        (PinIndexing::NoIndex, PinIndexing::Range(di, dj)) => {
            if di != dj {
                return Err(BuildError::InvalidRangeConection);
            }
            (0, di, 1)
        }
        (PinIndexing::Index(s), PinIndexing::NoIndex) => (s, 0, 1),
        (PinIndexing::Index(s), PinIndexing::Index(d)) => (s, d, 1),
        (PinIndexing::Index(s), PinIndexing::Range(di, dj)) => {
            if di != dj {
                return Err(BuildError::InvalidRangeConection);
            }
            (s, di, 1)
        }
    };

    Ok((src_idx as usize, dest_idx as usize, len as usize))
}

fn internal_name_to_idx(
    pin: &PinAddr,
    comp: &CompDecl,
    comp_map: &HashMap<String, Box<CompDecl>>,
    from_inputs: bool,
) -> Result<PinAddr, BuildError> {
    if let PinAddr::InternalName(name, pin, bidxs) = pin {
        debug!("|  Resolving internal pin: ({}, {})", name, pin);
        let comp_decl = get_comp_decl(comp, name, comp_map)?;

        let idx = if from_inputs {
            comp_decl
                .ins
                .get(pin)
                .ok_or(BuildError::InputPinNotFoundError(pin.clone()))
        } else {
            comp_decl
                .outs
                .get(pin)
                .ok_or_else(|| BuildError::OutputPinNotFoundError(pin.clone()))
        }?
        .0;

        debug!("|    Internal ({}, {}) resolved to idx: {}", name, pin, idx);
        Ok(PinAddr::InternalIdx(name.to_string(), idx, *bidxs))
    } else {
        Ok(pin.clone())
    }
}

fn get_comp_decl<'a>(
    comp: &'a CompDecl,
    name: &'a String,
    comp_map: &'a HashMap<String, Box<CompDecl>>,
) -> Result<&'a CompDecl, BuildError> {
    let subc = comp
        .subc
        .get(name)
        .ok_or(BuildError::ComponentRefNotFoundError(name.clone()))?;
    let subc_type = match subc {
        Comp::Primitive(prim) => prim.to_string(),
        Comp::Composite(name) => name.clone(),
    };
    let comp_decl = comp_map
        .get(&subc_type)
        .ok_or(BuildError::ComponentDeclNotFoundError(subc_type.clone()))?;
    Ok(comp_decl)
}
