use crate::component::{Component, ComponentBuilder};

pub struct LogicGate;

impl LogicGate {
    pub fn not(id: u32) -> Component {
        ComponentBuilder::new()
            .id(id)
            .upd_fn(|comp| {
                comp.outs[0] = !comp.ins[0];
            })
            .build()
    }

    pub fn and(id: u32, in_count: usize) -> Component {
        ComponentBuilder::new()
            .id(id)
            .upd_fn(|comp| {
                let out: bool = comp.ins.as_slice().iter().all(|val| *val);
                comp.outs[0] = out;
            })
            .input_count(in_count)
            .build()
    }

    pub fn or(id: u32, in_count: usize) -> Component {
        ComponentBuilder::new()
            .id(id)
            .upd_fn(|comp| {
                let out: bool = comp.ins.as_slice().iter().any(|val| *val);
                comp.outs[0] = out;
            })
            .input_count(in_count)
            .build()
    }

    pub fn nand(id: u32, in_count: usize) -> Component {
        ComponentBuilder::new()
            .id(id)
            .upd_fn(|comp| {
                let out: bool = comp.ins.as_slice().iter().all(|val| *val);
                comp.outs[0] = !out;
            })
            .input_count(in_count)
            .build()
    }

    pub fn nor(id: u32, in_count: usize) -> Component {
        ComponentBuilder::new()
            .id(id)
            .upd_fn(|comp| {
                let out: bool = comp.ins.as_slice().iter().any(|val| *val);
                comp.outs[0] = !out;
            })
            .input_count(in_count)
            .build()
    }

    pub fn xor(id: u32, in_count: usize) -> Component {
        ComponentBuilder::new()
            .id(id)
            .upd_fn(|comp| {
                let mut out = false;
                for i in 1..comp.ins.len() {
                    if comp.ins[i - 1] != comp.ins[i] {
                        out = true;
                        break;
                    }
                }
                comp.outs[0] = out;
            })
            .input_count(in_count)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::LogicGate;
    use crate::component::Component;

    fn test_gate(comp: &mut Component, table: Vec<Vec<bool>>) {
        for row in table {
            let last_idx = row.len() - 1;
            let ins = row[0..last_idx].to_vec();
            let out = row[last_idx];
            comp.set_ins(ins).update();
            assert!(comp.outs[0] == out, "Table: {:?}", row)
        }
    }

    #[test]
    fn not_gate() {
        test_gate(
            &mut LogicGate::not(0),
            vec![vec![false, true], vec![true, false]],
        );
    }

    #[test]
    fn and_gate() {
        test_gate(
            &mut LogicGate::and(0, 2),
            vec![
                vec![false, false, false],
                vec![true, false, false],
                vec![false, true, false],
                vec![true, true, true],
            ],
        );
    }

    #[test]
    fn or_gate() {
        test_gate(
            &mut LogicGate::or(0, 2),
            vec![
                vec![false, false, false],
                vec![true, false, true],
                vec![false, true, true],
                vec![true, true, true],
            ],
        );
    }

    #[test]
    fn nand_gate() {
        test_gate(
            &mut LogicGate::nand(0, 2),
            vec![
                vec![false, false, true],
                vec![true, false, true],
                vec![false, true, true],
                vec![true, true, false],
            ],
        );
    }

    #[test]
    fn nor_gate() {
        test_gate(
            &mut LogicGate::nor(0, 2),
            vec![
                vec![false, false, true],
                vec![true, false, false],
                vec![false, true, false],
                vec![true, true, false],
            ],
        );
    }

    #[test]
    fn xor_gate() {
        test_gate(
            &mut LogicGate::xor(0, 2),
            vec![
                vec![false, false, false],
                vec![true, false, true],
                vec![false, true, true],
                vec![true, true, false],
            ],
        );
    }
}
