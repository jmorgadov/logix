use logix_core::prelude::*;
use logix_sim::{bit::Bit, flattener::FlattenComponent, primitives::prelude::*, Simulation};

// TODO:
//  - (DONE) Flat component
//  - (PARTIALLY DONE) Stabilize algorithm
//  - Detect contradictions

fn sr_latch() -> Component<Bit> {
    ComponentBuilder::new("SR-Latch")
        .port_count(2, 2)
        .sub_comps(vec![nand_gate(2), nand_gate(2)])
        .connections(vec![Conn::new(0, 0, 1, 0), Conn::new(1, 0, 0, 1)])
        .in_addrs(vec![(0, (0, 0)), (1, (1, 1))])
        .out_addrs(vec![(0, 0), (1, 0)])
        .build()
}

fn jk_ff() -> Component<Bit> {
    ComponentBuilder::new("JK-Flip-Flop")
        .port_count(3, 2)
        .sub_comps(vec![and_gate(3), and_gate(3), sr_latch()])
        .connections(vec![
            Conn::new(0, 0, 2, 0),
            Conn::new(1, 0, 2, 1),
            Conn::new(2, 0, 1, 2),
            Conn::new(2, 1, 0, 0),
        ])
        .in_addrs(vec![(0, (0, 1)), (1, (0, 2)), (1, (1, 0)), (2, (1, 1))])
        .out_addrs(vec![(2, 0), (2, 1)])
        .build()
}


fn ms_jk() -> Component<Bit> {
    ComponentBuilder::new("MS-JK")
        .port_count(3, 2)
        .sub_comps(vec![
            nand_gate(3),
            nand_gate(3),
            sr_latch(),
            nand_gate(2),
            nand_gate(2),
            sr_latch(),
            not_gate(),
        ])
        .connections(vec![
            Conn::new(0, 0, 2, 0),
            Conn::new(1, 0, 2, 1),
            Conn::new(2, 0, 3, 0),
            Conn::new(2, 1, 4, 1),
            Conn::new(3, 0, 5, 0),
            Conn::new(4, 0, 5, 1),
            Conn::new(5, 0, 1, 2),
            Conn::new(5, 1, 0, 0),
            Conn::new(6, 0, 3, 1),
            Conn::new(6, 0, 4, 0),
        ])
        .in_addrs(vec![
            (0, (0, 1)),
            (1, (0, 2)),
            (1, (1, 0)),
            (1, (6, 0)),
            (2, (1, 1)),
        ])
        .out_addrs(vec![(5, 0), (5, 1)])
        .build()
}

fn main() {
    let counter = ComponentBuilder::new("Counter")
        .port_count(1, 1)
        .sub_comps(vec![
            high_const(),
            jk_ff(),
            jk_ff(),
            jk_ff(),
        ])
        .connections(vec![
            // Connect high constant
            Conn::new(0, 0, 1, 0),
            Conn::new(0, 0, 1, 2),
            Conn::new(0, 0, 2, 0),
            Conn::new(0, 0, 2, 2),
            Conn::new(0, 0, 3, 0),
            Conn::new(0, 0, 3, 2),
            // Connect each JK flip flop Q output to next JK flip flop clock
            Conn::new(1, 0, 2, 1),
            Conn::new(2, 0, 3, 1),
        ])
        .in_addrs(vec![(0, (1, 1))])
        .out_addrs(vec![(1, 0), (2, 0), (3, 0)])
        // .out_addrs(vec![(1, 0)])
        .build();

    let comp = ComponentBuilder::new("Main")
        .sub_comps(vec![clock(1.0), counter])
        .connections(vec![Conn::new(0, 0, 1, 0)])
        .build();

    let flat = FlattenComponent::new(comp);
    let mut sim = Simulation::new(flat);
    sim.start();
}
