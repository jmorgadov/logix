use crate::bit::{Bit, fmt_bit};
use logix_core::prelude::*;

#[derive(Debug)]
pub struct FlattenComponent {
    pub components: Vec<Component<Bit>>,
    pub connections: Vec<Conn>,

    pub deps: Vec<Vec<usize>>,
    pub inv_deps: Vec<Vec<usize>>,
}

impl FlattenComponent {
    pub fn new(comp: Component<Bit>) -> Self {
        let mut mut_comp = comp;
        reindex_connections(&mut mut_comp, 0);
        let (components, connections) = flat_comp(mut_comp);

        // Build dependency map
        let mut deps_mat: Vec<Vec<bool>> = vec![vec![false; components.len()]; components.len()];
        let mut inv_deps_mat: Vec<Vec<bool>> =
            vec![vec![false; components.len()]; components.len()];
        for conn in &connections {
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

        FlattenComponent {
            components,
            connections,
            deps,
            inv_deps,
        }
    }

    pub fn show(&self) {
        for (i, comp) in self.components.iter().enumerate() {
            print!("{} - ", i);
            show_comp(comp);
        }
    }
}

fn show_comp(comp: &Component<Bit>) {
    let mut line = String::from(&comp.name);
    line.push(' ');
    for bit in &comp.inputs {
        line.push(fmt_bit(bit));
    }
    line.push(' ');
    for bit in &comp.outputs {
        line.push(fmt_bit(bit));
    }
    println!("{}", line);

}

fn flat_comp(comp: Component<Bit>) -> (Vec<Component<Bit>>, Vec<Conn>) {
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
    (vec![comp], vec![])
}

fn reindex_connections(comp: &mut Component<Bit>, start_idx: usize) -> usize {
    if let Some(sub) = comp.sub.as_mut() {
        let mut idx_starts = vec![start_idx];
        for comp in sub.components.as_mut_slice() {
            let new_start = reindex_connections(comp, *idx_starts.last().unwrap());
            idx_starts.push(new_start);
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

        // change out addrs
        for i in 0..sub.out_addrs.len() {
            let (from_comp_idx, from_comp_addr) = sub.out_addrs[i];
            if let Some(subsub) = &sub.components[from_comp_idx].sub {
                sub.out_addrs[i] = subsub.out_addrs[from_comp_addr];
            } else {
                sub.out_addrs[i] = (idx_starts[from_comp_idx], from_comp_addr)
            }
        }

        return *idx_starts.last().unwrap();
    }
    start_idx + 1
}
