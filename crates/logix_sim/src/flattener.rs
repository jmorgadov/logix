use logix_core::prelude::*;

use crate::bit::Bit;

// TODO:
//  - (DONE) Flat component
//  - Stabilize algorithm
//  - Detect contradictions

pub fn flat_comp(comp: Component<Bit>) -> (Vec<Component<Bit>>, Vec<Conn>) {
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

pub fn reindex_connections(comp: &mut Component<Bit>, start_idx: usize) -> usize {
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
            // fix from
            if let Some(subsub) = &sub.components[conn.from.0].sub {
                (from_idx, from_addr) = subsub.out_addrs[conn.from.1];
            } else {
                from_idx = idx_starts[conn.from.0];
                from_addr = conn.from.1;
            }
            // fix to
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
        // let mut new_in_addrs = vec![];
        for i in 0..sub.in_addrs.len() {
            let (in_idx, (to_comp_idx, to_comp_addr)) = sub.in_addrs[i];
            if let Some(subsub) = &sub.components[to_comp_idx].sub {
                sub.in_addrs[i] = (in_idx, subsub.in_addrs[to_comp_addr].1);
            } else {
                sub.in_addrs[i] = (in_idx, (idx_starts[to_comp_idx], to_comp_addr));
            }
        }

        // change out addrs
        for i in 0..sub.out_addrs.len() {
            let (from_comp_idx, from_comp_addr) = sub.out_addrs[i];
            if let Some(subsub) = &sub.components[from_comp_idx].sub {
                sub.out_addrs[i] = subsub.out_addrs[from_comp_addr];
            } else {
                sub.out_addrs[i] = (idx_starts[from_comp_idx], from_comp_addr)
            }
        }

        println!("{:?}", idx_starts);
        return *idx_starts.last().unwrap();
    }
    start_idx + 1
}

// enum Dir {
//     Single(usize),
//     Compose(String, Vec<Dir>, Vec<(usize, PortAddr)>, Vec<PortAddr>),
// }

// impl Dir {
//     fn shift(self, n: usize) -> Dir {
//         match self {
//             Dir::Single(i) => Dir::Single(i + n),
//             Dir::Compose(name, dirs, ins, outs) => Dir::Compose(
//                 name,
//                 dirs.into_iter().map(|d| d.shift(n)).collect(),
//                 ins,
//                 outs,
//             ),
//         }
//     }
// }

// fn find_out_dir(port_addr: PortAddr, dir: Dir) -> PortAddr {
//     match dir {
//         Dir::Single(i) => (i, port_addr.1),
//         Dir::Compose(_, dirs, _, out_addrs) => {
//             let idx = port_addr.0;
//             let addr = port_addr.1;

//         },
//     }
// }

// fn reindex_conn(conn: Conn, flatten: Vec<Component<Bit>>, dir: Dir) -> Vec<Conn> {
//     let from_idx = conn.from.0;
//     let from_addr = conn.from.1;
//     let to_idx = conn.to.0;
//     let to_addr = conn.to.1;

// }

// fn flat_comp(
//     comp: Component<Bit>,
// ) -> (
//     Vec<Component<Bit>>,
//     Dir,
//     Vec<Conn>,
//     Vec<(usize, PortAddr)>,
//     Vec<PortAddr>,
// ) {
//     if let Some(sub) = comp.sub {
//         // Is composed component
//         let mut flatten: Vec<Component<Bit>> = vec![];
//         let mut dirs = vec![];
//         let mut conns = vec![];
//         for (mut flat_sub, dir, conn, inputs, outputs) in sub.components.into_iter().map(flat_comp)
//         {
//             dirs.push(dir.shift(flatten.len()));
//             flatten.append(&mut flat_sub);
//         }
//         let new_dir = Dir::Compose(comp.name, dirs);

//         // (flatten, new_dir, vec![])
//         todo!()
//     }

//     // Is primitive
//     let inputs: Vec<(usize, PortAddr)> = comp
//         .inputs
//         .iter()
//         .enumerate()
//         .map(|(i, _)| (i, (0, i)))
//         .collect();
//     let outputs: Vec<PortAddr> = comp
//         .outputs
//         .iter()
//         .enumerate()
//         .map(|(i, _)| (0, i))
//         .collect();
//     (vec![comp], Dir::Single(0), vec![], inputs, outputs)

//     //     let flatten: Vec<Component<Bit>> = vec![];
//     //     let dir = Dir::Single(0);

//     //     (flatten, dir)
// }

// // #[derive(Default)]
// // pub struct Simulation {
// //     comps: Vec<Component<Bit>>,
// //     conn: Vec<Conn>,
// // }

// // impl Simulation {
// //     pub fn new(comp: Component<Bit>) -> Self {
// //         let mut sim: Simulation = Default::default();
// //         sim.unfold_somp(comp);
// //         sim
// //     }

// //     fn unfold_somp(
// //         &mut self,
// //         comp: Component<Bit>,
// //     ) -> (Vec<Component<Bit>>, Vec<(usize, PortAddr)>, Vec<PortAddr>) {
// //         if let Some(sub) = &comp.sub {
// //             // Is composed component
// //             todo!()
// //         } else {
// //             // Is primitive
// //             let i_addrs: Vec<(usize, PortAddr)> = comp
// //                 .inputs
// //                 .iter()
// //                 .enumerate()
// //                 .map(|(i, _)| (i, (0, i)))
// //                 .collect();
// //             let o_addrs: Vec<PortAddr> = comp
// //                 .outputs
// //                 .iter()
// //                 .enumerate()
// //                 .map(|(i, _)| (0, i))
// //                 .collect();
// //             let c = vec![comp];
// //             (c, i_addrs, o_addrs)
// //         }
// //     }
// // }
