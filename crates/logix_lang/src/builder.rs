use log::debug;
use std::collections::HashMap;
use thiserror::Error;

use logix_core::component::{Component, ComponentBuilder, Conn, PortAddr};
use logix_sim::{
    bit::BitArray,
    primitives::primitive_builders::{
        and_gate, clock, high_const, low_const, nand_gate, not_gate, or_gate, xor_gate, BaseExtra,
    },
};

use crate::ast::prelude::*;

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
}

pub fn circuit_to_comp(circuit: Circuit) -> Result<Component<BitArray, BaseExtra>, BuildError> {
    let main = circuit
        .comps
        .iter()
        .find(|c| c.name == "Main")
        .ok_or(BuildError::NoMainComponentError)?;

    // Create a map of component names to component declarations
    // This is used to resolve the sub-components of a composite component
    let comp_map: HashMap<String, &CompDecl> = circuit
        .comps
        .iter()
        .map(|comp| (comp.name.clone(), comp))
        .collect();

    println!("{:?}", comp_map);

    return comp_decl_to_comp(main, &comp_map);
}

fn comp_decl_to_comp(
    comp: &CompDecl,
    comp_map: &HashMap<String, &CompDecl>,
) -> Result<Component<BitArray, BaseExtra>, BuildError> {
    let subc_map: HashMap<String, usize> = comp
        .subc
        .iter()
        .enumerate()
        .map(|(idx, (name, _))| (name.clone(), idx))
        .collect();

    let subc: Vec<Component<BitArray, BaseExtra>> = comp
        .subc
        .iter()
        .map(|(_, sub_comp)| {
            let sub_c = match sub_comp {
                Comp::Primitive(prim) => Ok(match prim {
                    Primitive::And(ins_count) => and_gate(*ins_count),
                    Primitive::Or(ins_count) => or_gate(*ins_count),
                    Primitive::Not => not_gate(),
                    Primitive::Nand(ins_count) => nand_gate(*ins_count),
                    Primitive::Nor(ins_count) => nand_gate(*ins_count),
                    Primitive::HighConst => high_const(),
                    Primitive::LowConst => low_const(),
                    Primitive::Clock(frec) => clock(*frec),
                    Primitive::Xor(ins_count) => xor_gate(*ins_count),
                }),
                Comp::Composite(name) => {
                    let decl = comp_map
                        .get(name)
                        .ok_or(BuildError::ComponentDeclNotFoundError(name.to_string()))?;
                    comp_decl_to_comp(decl, comp_map)
                }
            };
            sub_c
        })
        .collect::<Result<Vec<Component<BitArray, BaseExtra>>, BuildError>>()?;

    let (in_addrs, out_addrs, conns) = get_connections(comp, &subc, &subc_map, comp_map)?;

    Ok(ComponentBuilder::new(comp.name.as_str())
        .port_count(comp.ins.len(), comp.outs.len())
        .sub_comps(subc)
        .connections(conns)
        .in_addrs(in_addrs)
        .out_addrs(out_addrs)
        .build())
}

fn get_connections(
    comp: &CompDecl,
    subc: &Vec<Component<BitArray, BaseExtra>>,
    subc_map: &HashMap<String, usize>,
    comp_map: &HashMap<String, &CompDecl>,
) -> Result<(Vec<(usize, PortAddr)>, Vec<PortAddr>, Vec<Conn>), BuildError> {
    debug!("Processing connections for: {}", comp.name);

    let mut in_addrs = vec![];
    let mut out_addrs = vec![(0, 0); comp.outs.len()];
    let mut conns = vec![];

    let get_subc_idx = |name: &str| -> Result<usize, BuildError> {
        subc_map
            .get(name)
            .ok_or(BuildError::ComponentRefNotFoundError(name.to_string()))
            .map(|x| *x)
    };

    for conn in &comp.design {
        let src = internal_name_to_idx(&conn.src, comp, comp_map, false)?;
        let dest = internal_name_to_idx(&conn.dest, comp, comp_map, true)?;

        match (src, dest) {
            //
            // From input pin to internal component
            (PinAddr::External(in_name), PinAddr::InternalIdx(dest_name, idx)) => {
                debug!(
                    "|  Input pin to internal: ({}, ({}, {}))",
                    in_name, dest_name, idx
                );
                let in_idx = comp
                    .ins
                    .iter()
                    .position(|x| x == &in_name)
                    .ok_or(BuildError::InputPinNotFoundError(in_name.to_string()))?;
                let subc_idx = get_subc_idx(&dest_name)?;

                // Check if the idx is in range
                if subc[subc_idx].inputs.len() <= idx {
                    return Err(BuildError::InternalPinError(dest_name.clone(), idx));
                }

                in_addrs.push((in_idx, (subc_idx, idx)));
                debug!("|    In addr created: {:?}", in_addrs.last().unwrap());
            }
            //
            // From internal component to output pin
            (PinAddr::InternalIdx(src_name, idx), PinAddr::External(out_name)) => {
                debug!(
                    "|  Internal to output pin: (({}, {}), {})",
                    src_name, idx, out_name
                );
                let out_idx = comp
                    .outs
                    .iter()
                    .position(|x| x == &out_name)
                    .ok_or(BuildError::OutputPinNotFoundError(out_name.to_string()))?;

                let subc_idx = get_subc_idx(&src_name)?;

                // Check if the idx is in range
                if subc[subc_idx].outputs.len() <= idx {
                    return Err(BuildError::InternalPinError(src_name.clone(), idx));
                }

                out_addrs[out_idx] = (subc_idx, idx);
                debug!("|    Out addr created: {:?}", out_addrs.last().unwrap());
            }
            //
            // From internal to internal component
            (
                PinAddr::InternalIdx(src_name, src_idx),
                PinAddr::InternalIdx(dest_name, dest_idx),
            ) => {
                conns.push(Conn::new(
                    get_subc_idx(&src_name)?,
                    src_idx,
                    get_subc_idx(&dest_name)?,
                    dest_idx,
                ));
                debug!(
                    "|  Internal to internal: (({}, {}), ({}, {}))",
                    src_name, src_idx, dest_name, dest_idx
                );
                debug!("|    Connection created: {:?}", conns.last().unwrap())
            }
            (PinAddr::External(_), PinAddr::External(_)) => {
                return Err(BuildError::ExternalToExternalConnectionError);
            }
            _ => unreachable!(),
        }
    }

    Ok((in_addrs, out_addrs, conns))
}

fn internal_name_to_idx(
    pin: &PinAddr,
    comp: &CompDecl,
    comp_map: &HashMap<String, &CompDecl>,
    from_inputs: bool,
) -> Result<PinAddr, BuildError> {
    if let PinAddr::InternalName(name, pin) = pin {
        debug!("|  Resolving internal pin: ({}, {})", name, pin);
        let comp_decl = get_comp_decl(comp, name, comp_map)?;

        let idx = if from_inputs {
            comp_decl
                .ins
                .iter()
                .position(|x| x == pin)
                .ok_or_else(|| BuildError::InputPinNotFoundError(pin.clone()))
        } else {
            comp_decl
                .outs
                .iter()
                .position(|x| x == pin)
                .ok_or_else(|| BuildError::OutputPinNotFoundError(pin.clone()))
        }?;

        debug!("|    Internal ({}, {}) resolved to idx: {}", name, pin, idx);
        Ok(PinAddr::InternalIdx(name.to_string(), idx))
    } else {
        Ok(pin.clone())
    }
}

fn get_comp_decl<'a>(
    comp: &'a CompDecl,
    name: &'a String,
    comp_map: &'a HashMap<String, &'a CompDecl>,
) -> Result<&'a &'a CompDecl, BuildError> {
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
