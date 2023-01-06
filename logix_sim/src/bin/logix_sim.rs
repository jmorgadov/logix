use logix::prelude::*;
use logix_sim::{primitives::*, Simulation};

fn main() {
    let sr_latch = ComponentBuilder::new("SRLatch")
        .port_count(2, 2)
        .sub_comps(vec![nor_gate(2), nor_gate(2)])
        .connections(vec![Conn::new(0, 0, 1, 0), Conn::new(1, 0, 0, 1)])
        .in_addrs(vec![(0, 0), (1, 1)])
        .out_addrs(vec![(0, 0), (1, 0)])
        .build();

    let comp = ComponentBuilder::new("Main")
        .out_count(2)
        .sub_comps(vec![clock(1.0), clock(4.0), sr_latch])
        .connections(vec![Conn::new(0, 0, 2, 0), Conn::new(1, 0, 2, 1)])
        .out_addrs(vec![(2, 0), (2, 1)])
        .build();

    let mut sim = Simulation::new(comp);
    sim.start();
}
