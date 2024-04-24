use std::collections::HashMap;

use crate::{
    bit::{fmt_bit, Bit},
    primitives::{
        prelude::Primitive,
        primitives::{ExtraInfo, PrimitiveComponent},
    },
};
use logix_core::prelude::*;
use thiserror::Error;

#[derive(Debug)]
pub enum NestedConfig {
    Single(usize),
    Compose(
        String,
        HashMap<String, NestedConfig>,
        Vec<PortAddr>,
        Vec<PortAddr>,
    ),
}

#[derive(Debug, Error)]
pub enum FlattenError {}

#[derive(Debug)]
pub struct FlattenComponent {
    pub components: Vec<PrimitiveComponent>,
    pub connections: Vec<Vec<Conn>>,
    pub deps: Vec<Vec<usize>>,
    pub inv_deps: Vec<Vec<usize>>,

    pub nested_config: NestedConfig,
}

impl FlattenComponent {
    pub fn new(mut comp: Component<Bit, ExtraInfo>) -> Result<Self, FlattenError> {
        let (_, nested_config) = reindex_connections(&mut comp, 0)?;
        let (components, conns) = flat_comp(comp);

        // Build dependency map
        let mut deps_mat: Vec<Vec<bool>> = vec![vec![false; components.len()]; components.len()];
        let mut inv_deps_mat: Vec<Vec<bool>> =
            vec![vec![false; components.len()]; components.len()];

        for conn in &conns {
            let (from, to) = (idx_of(conn.from), idx_of(conn.to));
            deps_mat[to][from] = true;
            inv_deps_mat[from][to] = true;
        }

        let deps: Vec<Vec<usize>> = deps_mat
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(i, v)| if *v { Some(i) } else { None })
                    .collect()
            })
            .collect();
        let inv_deps: Vec<Vec<usize>> = inv_deps_mat
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(i, v)| if *v { Some(i) } else { None })
                    .collect()
            })
            .collect();

        let mut connections = components
            .iter()
            .map(|_| vec![])
            .collect::<Vec<Vec<Conn>>>();

        for conn in conns.into_iter() {
            connections[conn.from.0].push(conn);
        }

        Ok(FlattenComponent {
            components,
            connections,
            deps,
            inv_deps,
            nested_config,
        })
    }

    pub fn get_status(&self, comp_path: &str) -> (Vec<bool>, Vec<bool>) {
        let mut comp = &self.nested_config;
        for name in comp_path.split('.') {
            match comp {
                NestedConfig::Compose(_, subs, _, _) => {
                    comp = subs.get(name).unwrap();
                }
                _ => panic!("Component not found"),
            }
        }

        match comp {
            NestedConfig::Compose(_, _, ins, outs) => {
                let mut in_bits = vec![];
                let mut out_bits = vec![];
                for addr in ins {
                    in_bits.push(self.components[addr.0].inputs[addr.1]);
                }
                for addr in outs {
                    out_bits.push(self.components[addr.0].outputs[addr.1]);
                }
                (in_bits, out_bits)
            }
            NestedConfig::Single(idx) => {
                let comp = &self.components[*idx];
                (comp.inputs.clone(), comp.outputs.clone())
            }
        }
    }

    pub fn show_status_of(&self, comp_path: &str) {
        println!("{:?}", comp_path);
        let (in_bits, out_bits) = self.get_status(comp_path);
        for bit in in_bits.iter() {
            print!("{}", fmt_bit(bit));
        }
        print!(" -> ");
        for bit in out_bits.iter() {
            print!("{}", fmt_bit(bit));
        }
        println!();
    }

    pub fn show_flat(&self) {
        for (i, comp) in self.components.iter().enumerate() {
            print!("{} - ", i);
            show_comp(comp);
        }
    }

    fn show_config(&self, conf: &NestedConfig, level: usize) -> String {
        let mut s = String::new();
        match conf {
            NestedConfig::Compose(n, subs, ins, outs) => {
                for _ in 0..level {
                    s.push_str("  ");
                }
                s.push_str(n);
                s.push(' ');
                for addr in ins {
                    s.push(fmt_bit(&self.components[addr.0].inputs[addr.1]));
                }
                s.push(' ');
                for addr in outs {
                    s.push(fmt_bit(&self.components[addr.0].outputs[addr.1]));
                }
                s.push('\n');
                subs.iter().for_each(|c| {
                    s.push_str(&self.show_config(c.1, level + 1));
                });
            }
            NestedConfig::Single(idx) => {
                let comp = &self.components[*idx];
                for _ in 0..level {
                    s.push_str("  ");
                }
                s.push_str(&comp.name);
                s.push(' ');
                for bit in &comp.inputs {
                    s.push(fmt_bit(bit));
                }
                s.push(' ');
                for bit in &comp.outputs {
                    s.push(fmt_bit(bit));
                }
                s.push('\n');
            }
        }
        s
    }

    pub fn show(&self) {
        println!("{}", self.show_config(&self.nested_config, 0));
    }
}

fn show_comp(comp: &PrimitiveComponent) {
    let mut line = String::from(&comp.name);
    line.push(' ');
    for bits in &comp.inputs {
        line.push(fmt_bit(bits));
    }
    line.push(' ');
    for bits in &comp.outputs {
        line.push(fmt_bit(bits));
    }
    println!("{}", line);
}

fn flat_comp(comp: Component<Bit, ExtraInfo>) -> (Vec<PrimitiveComponent>, Vec<Conn>) {
    let mut comps = vec![];
    let mut conns = vec![];
    if let Some(mut sub) = comp.sub {
        conns.append(&mut sub.connections);
        for (mut flatten, mut conn) in sub.components.into_iter().map(flat_comp) {
            comps.append(&mut flatten);
            conns.append(&mut conn);
        }
        return (comps, conns);
    }

    assert!(comp.extra.primitive.is_some());

    let in_count = comp.inputs.len();
    let new_comp = match comp.extra.primitive.unwrap() {
        Primitive::AndGate => PrimitiveComponent::and_gate(in_count),
        Primitive::OrGate => PrimitiveComponent::or_gate(in_count),
        Primitive::NotGate => PrimitiveComponent::not_gate(),
        Primitive::NandGate => PrimitiveComponent::nand_gate(in_count),
        Primitive::NorGate => PrimitiveComponent::nor_gate(in_count),
        Primitive::XorGate => PrimitiveComponent::xor_gate(in_count),
        Primitive::Clock { period } => PrimitiveComponent::clock(period),
        Primitive::Const { value } => PrimitiveComponent::const_gate(value),
    };

    (vec![new_comp], vec![])
}

fn reindex_connections(
    comp: &mut Component<Bit, ExtraInfo>,
    start_idx: usize,
) -> Result<(usize, NestedConfig), FlattenError> {
    if comp.sub.is_none() {
        return Ok((start_idx + 1, NestedConfig::Single(start_idx)));
    }
    let sub = comp.sub.as_mut().unwrap();
    let mut idx_starts = vec![start_idx];
    let mut sub_configs = HashMap::new();

    for comp in sub.components.as_mut_slice() {
        let (new_start, config) = reindex_connections(comp, *idx_starts.last().unwrap())?;
        idx_starts.push(new_start);
        sub_configs.insert(comp.extra.id.to_string(), config);
    }

    // change connections
    let mut new_conns = vec![];
    for i in 0..sub.connections.len() {
        let conn = &sub.connections[i];
        let from_idx: usize;
        let from_addr: usize;
        let mut to_ports = vec![];

        // reindex from
        if let Some(subsub) = &sub.components[conn.from.0].sub {
            (from_idx, from_addr) = subsub.out_addrs[conn.from.1];
        } else {
            from_idx = idx_starts[conn.from.0];
            from_addr = conn.from.1;
        }

        // reindex to
        if let Some(subsub) = &sub.components[conn.to.0].sub {
            for (in_idx, (comp_idx, comp_addr)) in &subsub.in_addrs {
                if *in_idx == conn.to.1 {
                    to_ports.push((*comp_idx, *comp_addr));
                }
            }
        } else {
            to_ports = vec![(idx_starts[conn.to.0], conn.to.1)];
        }
        for to_port in to_ports {
            new_conns.push(Conn::new(from_idx, from_addr, to_port.0, to_port.1));
        }
    }
    sub.connections = new_conns;

    // change in addresses
    let mut new_in_addrs = vec![];
    for i in 0..sub.in_addrs.len() {
        let (in_idx, (to_comp_idx, to_comp_addr)) = sub.in_addrs[i];
        if let Some(subsub) = &sub.components[to_comp_idx].sub {
            for (j, (comp_idx, comp_addr)) in &subsub.in_addrs {
                if *j == to_comp_addr {
                    new_in_addrs.push((in_idx, (*comp_idx, *comp_addr)));
                }
            }
        } else {
            new_in_addrs.push((in_idx, (idx_starts[to_comp_idx], to_comp_addr)));
        }
    }
    sub.in_addrs = new_in_addrs;
    let max = sub.in_addrs.iter().map(|(i, _)| *i).max();
    let mut nested_in = vec![];
    if let Some(m) = max {
        nested_in.resize(m + 1, (0, 0));
        for in_addr in &sub.in_addrs {
            nested_in[in_addr.0] = in_addr.1;
        }
    }

    // change out addresses
    for i in 0..sub.out_addrs.len() {
        let (from_comp_idx, from_comp_addr) = sub.out_addrs[i];
        if let Some(subsub) = &sub.components[from_comp_idx].sub {
            sub.out_addrs[i] = subsub.out_addrs[from_comp_addr];
        } else {
            sub.out_addrs[i] = (idx_starts[from_comp_idx], from_comp_addr)
        }
    }

    let nested_out = sub.out_addrs.clone();
    let config = NestedConfig::Compose(comp.name.clone(), sub_configs, nested_in, nested_out);
    return Ok((*idx_starts.last().unwrap(), config));
}
