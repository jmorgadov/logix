use std::collections::HashMap;

use crate::{
    errors::{DataRequestError, FlattenComponentError},
    primitives::{
        data::Data,
        prelude::Primitive,
        primitive::{ExtraInfo, PrimitiveComponent},
    },
};
use log::*;
use logix_core::prelude::*;

#[derive(Debug)]
pub enum NestedConfig {
    Single(String, usize, Vec<PortAddr>, Vec<PortAddr>),
    Compose(
        String,
        usize,
        HashMap<usize, NestedConfig>,
        Vec<PortAddr>,
        Vec<PortAddr>,
    ),
}

impl NestedConfig {
    pub fn get_input_addr(&self, idx: usize) -> PortAddr {
        match self {
            NestedConfig::Single(_, _, ins, _) => ins[idx],
            NestedConfig::Compose(_, _, _, ins, _) => ins[idx],
        }
    }

    pub fn get_output_addr(&self, idx: usize) -> PortAddr {
        match self {
            NestedConfig::Single(_, _, _, outs) => outs[idx],
            NestedConfig::Compose(_, _, _, _, outs) => outs[idx],
        }
    }
}

#[derive(Debug)]
pub struct FlattenComponent {
    pub components: Vec<PrimitiveComponent>,
    pub connections: Vec<Vec<Conn>>,
    pub deps: Vec<Vec<usize>>,
    pub inv_deps: Vec<Vec<usize>>,

    pub nested_config: NestedConfig,

    id_to_idx: HashMap<usize, usize>,
}

impl FlattenComponent {
    pub fn new(mut comp: Component<ExtraInfo>) -> Result<Self, FlattenComponentError> {
        let original = comp.clone();
        let (_, mut nested_config) = reindex_connections(&mut comp, 0)?;
        fix_inputs_data_addrs(&original, &mut nested_config);

        let (components, conns) = flat_comp(&comp);

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

        let id_to_idx = components
            .iter()
            .enumerate()
            .map(|(i, comp)| (comp.id, i))
            .collect();

        Ok(FlattenComponent {
            components,
            connections,
            deps,
            inv_deps,
            nested_config,
            id_to_idx,
        })
    }

    pub fn comp_by_id(&self, id: usize) -> &PrimitiveComponent {
        &self.components[*self
            .id_to_idx
            .get(&id)
            .unwrap_or_else(|| panic!("Component {} not found", id))]
    }

    pub fn get_input_status_at(&self, id: usize, idx: usize) -> Result<Data, DataRequestError> {
        let c_idx = *self
            .id_to_idx
            .get(&id)
            .ok_or(DataRequestError::InvalidComponentId(id))?;
        self.components[c_idx]
            .inputs
            .get(idx)
            .copied()
            .ok_or(DataRequestError::InvalidInputPortIndex(idx))
    }

    pub fn get_output_status_at(&self, id: usize, idx: usize) -> Result<Data, DataRequestError> {
        let c_idx = *self
            .id_to_idx
            .get(&id)
            .ok_or(DataRequestError::InvalidComponentId(id))?;
        self.components[c_idx]
            .outputs
            .get(idx)
            .copied()
            .ok_or(DataRequestError::InvalidOutputPortIndex(idx))
    }

    pub fn get_status_by_id(&self, id: usize) -> (&Vec<Data>, &Vec<Data>) {
        let idx = *self.id_to_idx.get(&id).expect("Component not found");
        (&self.components[idx].inputs, &self.components[idx].outputs)
    }

    pub fn get_status(
        &self,
        comp_path: &[usize],
        at: Option<usize>,
    ) -> Result<(Vec<Data>, Vec<Data>), DataRequestError> {
        let mut comp = &self.nested_config;
        for id in comp_path {
            match comp {
                NestedConfig::Compose(_, _, subs, _, _) => {
                    comp = subs
                        .get(id)
                        .ok_or(DataRequestError::InvalidComponentId(*id))?;
                }
                _ => panic!("Component not found"),
            }
        }

        if let Some(at) = at {
            match comp {
                NestedConfig::Compose(_, _, subs, _, _) => {
                    comp = subs
                        .get(&at)
                        .ok_or(DataRequestError::InvalidComponentId(at))?;
                }
                _ => panic!("Component not found"),
            }
        }

        match comp {
            NestedConfig::Compose(_, _, _, ins, outs) => {
                let mut in_bits = vec![];
                let mut out_bits = vec![];
                for addr in ins {
                    in_bits.push(self.components[addr.0].inputs[addr.1]);
                }
                for addr in outs {
                    out_bits.push(self.components[addr.0].outputs[addr.1]);
                }
                Ok((in_bits, out_bits))
            }
            NestedConfig::Single(_, _, ins, outs) => {
                let mut in_bits = vec![];
                let mut out_bits = vec![];
                for addr in ins {
                    in_bits.push(self.components[addr.0].inputs[addr.1]);
                }
                for addr in outs {
                    out_bits.push(self.components[addr.0].outputs[addr.1]);
                }
                Ok((in_bits, out_bits))
            }
        }
    }
}

fn flat_comp(comp: &Component<ExtraInfo>) -> (Vec<PrimitiveComponent>, Vec<Conn>) {
    let mut comps = vec![];
    let mut conns = vec![];
    if let Some(sub) = &comp.sub {
        conns.extend(sub.connections.clone());
        for (flatten, conn) in sub.components.iter().map(flat_comp) {
            comps.extend(flatten);
            conns.extend(conn.clone());
        }
        return (comps, conns);
    }

    assert!(comp.extra.primitive.is_some());

    let in_count = comp.inputs;
    let id = comp.extra.id;
    let new_comp = match comp.extra.primitive.as_ref().unwrap() {
        Primitive::AndGate => PrimitiveComponent::and_gate(id, in_count),
        Primitive::OrGate => PrimitiveComponent::or_gate(id, in_count),
        Primitive::NotGate => PrimitiveComponent::not_gate(id),
        Primitive::NandGate => PrimitiveComponent::nand_gate(id, in_count),
        Primitive::NorGate => PrimitiveComponent::nor_gate(id, in_count),
        Primitive::XorGate => PrimitiveComponent::xor_gate(id, in_count),
        Primitive::Input { bits } => PrimitiveComponent::input(id, *bits),
        Primitive::Output { bits } => PrimitiveComponent::output(id, *bits),
        Primitive::Splitter { bits } => PrimitiveComponent::splitter(id, *bits),
        Primitive::Joiner { bits } => PrimitiveComponent::joiner(id, *bits),
        Primitive::Clock { period } => PrimitiveComponent::clock(id, *period),
        Primitive::Const { value } => PrimitiveComponent::const_gate(id, *value),
        Primitive::Custom { comp, state: _ } => PrimitiveComponent::custom(id, comp.clone()),
    };

    (vec![new_comp], vec![])
}

fn reindex_connections(
    comp: &mut Component<ExtraInfo>,
    start_idx: usize,
) -> Result<(usize, NestedConfig), FlattenComponentError> {
    debug!(
        "Reindexing connections for component: {:?} statring from {}",
        comp.name, start_idx
    );

    if comp.sub.is_none() {
        // is a primitive
        let ins = (0..comp.inputs).map(|i| (start_idx, i)).collect();
        let outs = (0..comp.outputs).map(|i| (start_idx, i)).collect();
        return Ok((
            start_idx + 1,
            NestedConfig::Single(comp.name.clone().unwrap_or_default(), comp.id, ins, outs),
        ));
    }

    let sub = comp.sub.as_mut().unwrap();
    let mut idx_starts = vec![start_idx];
    let mut sub_configs = HashMap::new();

    // reindex subcomponents
    for comp in sub.components.as_mut_slice() {
        let (new_start, config) = reindex_connections(comp, *idx_starts.last().unwrap())?;
        idx_starts.push(new_start);
        sub_configs.insert(comp.id, config);
    }

    // change connections
    debug!("Changing connections for component: {:?}", comp.name);
    let mut new_conns = vec![];
    for i in 0..sub.connections.len() {
        debug!("Changing connection: {:?}", sub.connections[i]);
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

        debug!("New src idx: ({}, {})", from_idx, from_addr);

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

        debug!("New dest idxs: {:?}", to_ports);

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
    let config = NestedConfig::Compose(
        comp.name.clone().unwrap_or_default(),
        comp.id,
        sub_configs,
        nested_in,
        nested_out,
    );

    return Ok((*idx_starts.last().unwrap(), config));
}

fn fix_inputs_data_addrs(comp: &Component<ExtraInfo>, config: &mut NestedConfig) {
    let Some(sub) = &comp.sub else {
        return;
    };
    let NestedConfig::Compose(_, _, subs, _, _) = config else {
        return;
    };

    let sub_comps = &sub.components;

    for conn in &sub.connections {
        let (from_idx, from_port) = conn.from;
        let (to_idx, to_port) = conn.to;

        let from_id = sub_comps[from_idx].id;
        let to_id = sub_comps[to_idx].id;

        let new_addr = subs[&from_id].get_output_addr(from_port);

        let NestedConfig::Compose(_, _, sub_subs, _, _) = subs.get_mut(&to_id).unwrap() else {
            continue;
        };

        let subsub = sub_comps[to_idx]
            .sub
            .as_ref()
            .expect("Subcomponent should not be primitive at this point");

        let in_comp = subsub
            .components
            .iter()
            .filter(|c| {
                c.extra
                    .primitive
                    .as_ref()
                    .is_some_and(|prim| matches!(prim, Primitive::Input { .. }))
            })
            .take(to_port + 1)
            .last()
            .expect("Input not found");

        let input_com_id = in_comp.id;
        let NestedConfig::Single(_, _, _, outs) =
            sub_subs.get_mut(&input_com_id).expect("Input not found")
        else {
            panic!("Component config is not single");
        };

        outs[0] = new_addr;
    }

    for comp in sub_comps {
        fix_inputs_data_addrs(comp, subs.get_mut(&comp.id).unwrap());
    }
}
