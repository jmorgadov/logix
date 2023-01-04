use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents a component that emits a constant value.
#[derive(Debug)]
pub struct Const {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl Const {
    /// Creates a new `Const` component given an id and the value that will
    /// emit.
    ///
    /// # Arguments
    ///
    /// * `value` - Bool that represents the constant value.
    ///
    /// # Example
    ///
    /// ```
    /// use logix::prelude::Const;
    /// let const_comp = Const::new(true);
    /// ```
    pub fn new(value: bool) -> Self {
        Const {
            ins: vec![],
            outs: vec![value],
        }
    }

    /// Creates a new `Const` component given an id with a value of true.
    ///
    /// # Example
    ///
    /// ```
    /// use logix::prelude::Const;
    /// let const_comp = Const::one();
    /// ```
    pub fn one() -> Self {
        Const::new(true)
    }

    /// Creates a new `Const` component given an id with a value of false.
    ///
    /// # Example
    ///
    /// ```
    /// use logix::prelude::Const;
    /// let const_comp = Const::zero();
    /// ```
    pub fn zero() -> Self {
        Const::new(false)
    }
}

impl ComponentCast for Const {
    fn as_const(&self) -> Option<&Const> {
        Some(self)
    }
}

impl Component for Const {
    fn name(&self) -> String {
        match self.outs[0] {
            true => Primitive::ConstOne.to_string(),
            false => Primitive::ConstZero.to_string(),
        }
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}

#[cfg(test)]
mod tests {
    use super::Const;
    use crate::components::component::{CompEvent, Component};

    #[test]
    fn cont_one() {
        let comp = &mut Const::one();
        comp.on_event(&CompEvent::UpdateValues);
        assert!(comp.outs[0]);
    }

    #[test]
    fn cont_zero() {
        let comp = &mut Const::zero();
        comp.on_event(&CompEvent::UpdateValues);
        assert!(!comp.outs[0]);
    }
}
