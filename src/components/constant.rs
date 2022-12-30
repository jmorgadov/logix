use super::component::{BaseComponent, ComponentBuilder};

pub struct Const;

impl Const {
    pub fn one(id: u32) -> BaseComponent {
        ComponentBuilder::new()
            .name("ConstantOne")
            .id(id)
            .upd_fn(|comp| {
                comp.outs[0] = true;
            })
            .default_in(0, true)
            .build()
    }

    pub fn zero(id: u32) -> BaseComponent {
        ComponentBuilder::new()
            .name("ConstantZero")
            .id(id)
            .upd_fn(|comp| {
                comp.outs[0] = false;
            })
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::Const;
    use crate::components::component::Component;

    #[test]
    fn cont_one() {
        let comp = &mut Const::one(0);
        comp.check_values();
        assert!(comp.outs[0]);
    }

    #[test]
    fn cont_zero() {
        let comp = &mut Const::zero(0);
        comp.check_values();
        assert!(!comp.outs[0]);
    }
}
