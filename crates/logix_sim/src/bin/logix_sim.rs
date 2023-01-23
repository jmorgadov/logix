use logix_core::prelude::*;
use logix_sim::{bit::Bit, flattener::FlattenComponent, primitives::prelude::*, Simulation};

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
        .sub_comps(vec![nand_gate(3), nand_gate(3), sr_latch()])
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
        .port_count(5, 2)
        .sub_comps(vec![
            nand_gate(3),
            nand_gate(3),
            nand_gate(2),
            nand_gate(2),
            nand_gate(2),
            nand_gate(2),
            nand_gate(3),
            nand_gate(3),
            nand_gate(2),
            nand_gate(2),
            nand_gate(2),
            nand_gate(2),
            not_gate(),
            not_gate(),
            not_gate(),
        ])
        .connections(vec![
            Conn::new(0, 0, 2, 0),
            Conn::new(1, 0, 3, 1),
            Conn::new(2, 0, 3, 0),
            Conn::new(3, 0, 2, 1),
            Conn::new(2, 0, 4, 0),
            Conn::new(3, 0, 5, 1),
            Conn::new(4, 0, 6, 1),
            Conn::new(5, 0, 7, 1),
            Conn::new(6, 0, 7, 0),
            Conn::new(7, 0, 6, 2),
            Conn::new(6, 0, 8, 1),
            Conn::new(7, 0, 9, 1),
            Conn::new(6, 0, 1, 2),
            Conn::new(7, 0, 0, 0),
            Conn::new(8, 0, 10, 0),
            Conn::new(8, 0, 10, 1),
            Conn::new(9, 0, 11, 0),
            Conn::new(9, 0, 11, 1),
            Conn::new(12, 0, 4, 1),
            Conn::new(12, 0, 5, 0),
            Conn::new(13, 0, 6, 0),
            Conn::new(13, 0, 9, 0),
            Conn::new(14, 0, 7, 2),
            Conn::new(14, 0, 8, 0),
        ])
        .in_addrs(vec![
            // J
            (0, (0, 1)),
            // CLK
            (1, (12, 0)),
            (1, (0, 2)),
            (1, (1, 0)),
            // K
            (2, (1, 1)),
            // PRESET
            (3, (13, 0)),
            // RESET
            (4, (14, 0)),
        ])
        .out_addrs(vec![
            // Q
            (10, 0),
            // !Q
            (11, 0),
        ])
        .build()
}

fn main() {
    let comp = ComponentBuilder::new("Main")
        .sub_comps(vec![clock(1.0)])
        .build();
    let flat = FlattenComponent::new(comp);
    let mut sim = Simulation::new(flat);
    sim.start();
}
