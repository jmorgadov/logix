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

    let in_addrs: Vec<(usize, PortAddr)> = comp
        .design
        .iter()
        .filter(|conn| {
            if let PinAddr::External(name) = &conn.src {
                assert!(
                    comp.ins.contains(&name),
                    "Input {} not found in component {}",
                    name,
                    comp.name
                );
                return true;
            }
            return false;
        })
        .map(|conn| {
            let src_idx = match &conn.src {
                PinAddr::External(name) => comp.ins.iter().position(|x| x == name).unwrap(),
                _ => panic!("Internal pin found in input"),
            };

            let dest: PortAddr = match &conn.dest {
                PinAddr::InternalName(comp_name, name) => {
                    let c = comp.subc.get(comp_name).unwrap();

                    let c_name = match c {
                        Comp::Primitive(prim) => prim.to_string(),
                        Comp::Composite(name) => name.clone(),
                    };
                    let comp = comp_map.get(&c_name).unwrap();
                    (
                        *subc_map.get(comp_name).unwrap(),
                        comp.ins.iter().position(|x| x == name).unwrap(),
                    )
                }
                PinAddr::InternalIdx(comp_name, idx) => {
                    (*subc_map.get(comp_name).unwrap(), *idx as usize)
                }
                PinAddr::External(_) => panic!("Invalid destination pin"),
            };
            (src_idx, dest)
        })
        .collect();

    let out_addrs: Vec<PortAddr> = comp
        .design
        .iter()
        .filter(|conn| {
            if let PinAddr::External(name) = &conn.dest {
                assert!(
                    comp.outs.contains(&name),
                    "Output {} not found in component {}",
                    name,
                    comp.name
                );
                return true;
            }
            return false;
        })
        .map(|conn| {
            let src: PortAddr = match &conn.src {
                PinAddr::InternalName(comp_name, name) => {
                    let c = comp.subc.get(comp_name).unwrap();

                    let c_name = match c {
                        Comp::Primitive(prim) => prim.to_string(),
                        Comp::Composite(name) => name.clone(),
                    };
                    let comp = comp_map.get(&c_name).unwrap();
                    (
                        *subc_map.get(comp_name).unwrap(),
                        comp.outs.iter().position(|x| x == name).unwrap(),
                    )
                }
                PinAddr::InternalIdx(comp_name, idx) => {
                    (*subc_map.get(comp_name).unwrap(), *idx as usize)
                }
                PinAddr::External(_) => panic!("Invalid input pin"),
            };
            src
        })
        .collect();

    let conns: Vec<Conn> = comp
        .design
        .iter()
        .filter(|conn| {
            if let PinAddr::External(_) = &conn.src {
                return false;
            }
            if let PinAddr::External(_) = &conn.dest {
                return false;
            }
            return true;
        })
        .map(|conn| {
            let src: PortAddr = match &conn.src {
                PinAddr::InternalName(comp_name, out_name) => {
                    let c = comp.subc.get(comp_name).unwrap();

                    let c_name = match c {
                        Comp::Primitive(prim) => prim.to_string(),
                        Comp::Composite(name) => name.clone(),
                    };
                    let comp = comp_map.get(&c_name).unwrap();
                    (
                        *subc_map.get(comp_name).unwrap(),
                        comp.outs.iter().position(|x| x == out_name).unwrap(),
                    )
                }
                PinAddr::InternalIdx(comp_name, idx) => (*subc_map.get(comp_name).unwrap(), *idx),
                _ => panic!("Unreashable"),
            };

            let dest: PortAddr = match &conn.dest {
                PinAddr::InternalName(comp_name, name) => {
                    let c = comp.subc.get(comp_name).unwrap();

                    let c_name = match c {
                        Comp::Primitive(prim) => prim.to_string(),
                        Comp::Composite(name) => name.clone(),
                    };
                    let comp = comp_map.get(&c_name).unwrap();
                    println!("{:?}", comp);
                    (
                        *subc_map.get(comp_name).unwrap(),
                        comp.ins.iter().position(|x| x == name).unwrap(),
                    )
                }
                PinAddr::InternalIdx(comp_name, idx) => (*subc_map.get(comp_name).unwrap(), *idx),
                _ => panic!("Unreashable"),
            };
            Conn::new(src.0, src.1, dest.0, dest.1)
        })
        .collect();

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
