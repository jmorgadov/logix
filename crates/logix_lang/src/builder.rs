use std::collections::HashMap;

use logix_core::component::{Component, ComponentBuilder, Conn, PortAddr};
use logix_sim::{
    bit::Bit,
    primitives::primitive_builders::{
        and_gate, clock, high_const, low_const, nand_gate, not_gate, or_gate, xor_gate,
    },
};

use crate::ast::prelude::*;

pub fn circuit_to_comp(circuit: Circuit) -> Component<Bit> {
    let main = circuit.comps.iter().find(|c| c.name == "Main").unwrap();

    let mut comp_map = HashMap::new();
    for comp in circuit.comps.iter() {
        comp_map.insert(comp.name.clone(), comp);
    }
    println!("{:?}", comp_map);
    return comp_decl_to_comp(main, &comp_map);
}

fn comp_decl_to_comp(comp: &CompDecl, comp_map: &HashMap<String, &CompDecl>) -> Component<Bit> {
    let mut subc_map = HashMap::new();
    let mut idx: usize = 0;
    for (name, _) in comp.subc.iter() {
        subc_map.insert(name.clone(), idx);
        idx += 1;
    }

    let (in_addrs, out_addrs, conns) = get_connections(comp, &subc_map, comp_map);

    let subc: Vec<Component<Bit>> = comp
        .subc
        .iter()
        .map(|(_, sub_comp)| {
            let sub_c = match sub_comp {
                Comp::Primitive(prim) => match prim {
                    Primitive::And(ins_count) => and_gate(*ins_count),
                    Primitive::Or(ins_count) => or_gate(*ins_count),
                    Primitive::Not => not_gate(),
                    Primitive::Nand(ins_count) => nand_gate(*ins_count),
                    Primitive::Nor(ins_count) => nand_gate(*ins_count),
                    Primitive::HighConst => high_const(),
                    Primitive::LowConst => low_const(),
                    Primitive::Clock(frec) => clock(*frec),
                    Primitive::Xor(ins_count) => xor_gate(*ins_count),
                },
                Comp::Composite(name) => comp_decl_to_comp(comp_map.get(name).unwrap(), comp_map),
            };
            sub_c
        })
        .collect();

    ComponentBuilder::new(comp.name.as_str())
        .port_count(comp.ins.len(), comp.outs.len())
        .sub_comps(subc)
        .connections(conns)
        .in_addrs(in_addrs)
        .out_addrs(out_addrs)
        .build()
}

fn get_connections(
    comp: &CompDecl,
    subc_map: &HashMap<String, usize>,
    comp_map: &HashMap<String, &CompDecl>,
) -> (Vec<(usize, PortAddr)>, Vec<PortAddr>, Vec<Conn>) {
    let mut in_addrs = vec![(0, (0, 0)); comp.ins.len()];
    let mut out_addrs = vec![(0, 0); comp.outs.len()];
    let mut conns = vec![];

    comp.design.iter().for_each(|conn| {
        let src = if let PinAddr::InternalName(name, pin) = &conn.src {
            let subc = comp.subc.get(name).unwrap();
            let subc_type = match subc {
                Comp::Primitive(prim) => prim.to_string(),
                Comp::Composite(name) => name.clone(),
            };
            let comp_decl = comp_map.get(&subc_type).unwrap();
            let idx = comp_decl.outs.iter().position(|x| x == pin).unwrap();
            PinAddr::InternalIdx(name.to_string(), idx)
        } else {
            conn.src.clone()
        };

        let dest = if let PinAddr::InternalName(name, pin) = &conn.dest {
            let subc = comp.subc.get(name).unwrap();
            let subc_type = match subc {
                Comp::Primitive(prim) => prim.to_string(),
                Comp::Composite(name) => name.clone(),
            };
            let comp_decl = comp_map.get(&subc_type).unwrap();
            let idx = comp_decl.ins.iter().position(|x| x == pin).expect("");

            PinAddr::InternalIdx(name.to_string(), idx)
        } else {
            conn.dest.clone()
        };

        match (src, dest) {
            //
            // From input pin to internal component
            (PinAddr::External(in_name), PinAddr::InternalIdx(dest_name, idx)) => {
                let in_idx = comp.ins.iter().position(|x| x == &in_name).unwrap();
                in_addrs[in_idx] = (in_idx, (*subc_map.get(&dest_name).unwrap(), idx));
            }
            //
            // From internal component to output pin
            (PinAddr::InternalIdx(src_name, idx), PinAddr::External(out_name)) => {
                let out_idx = comp.outs.iter().position(|x| x == &out_name).unwrap();
                out_addrs[out_idx] = (*subc_map.get(&src_name).unwrap(), idx);
            }
            //
            // From internal to internal component
            (
                PinAddr::InternalIdx(src_name, src_idx),
                PinAddr::InternalIdx(dest_name, dest_idx),
            ) => {
                conns.push(Conn::new(
                    *subc_map.get(&src_name).unwrap(),
                    src_idx,
                    *subc_map.get(&dest_name).unwrap(),
                    dest_idx,
                ));
            }
            (PinAddr::External(_), PinAddr::External(_)) => panic!("Invalid connection"),
            _ => unreachable!(),
        }
    });

    (in_addrs, out_addrs, conns)
}