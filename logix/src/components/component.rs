use std::fmt::Debug;

use super::prelude::ComponentCast;

/// Basic trait that describes a component.
///
/// # Implementation details
///
/// All components that implement the `Component` trait are consider primitives,
/// except for the `ComposedComponent`. It is a special implementation of this
/// trait which can hold several components (even composed ones) and connect them
/// to make a new bigger component.
pub trait Component: Debug + ComponentCast {
    /// Returns the name of the component.
    fn name(&self) -> String;

    /// Returns a vector of bool representing the input values of the component.
    fn ins(&mut self) -> &mut Vec<bool>;

    /// Returns a vector of bool representing the input values of the component.
    fn outs(&mut self) -> &mut Vec<bool>;

    /// Sets a value to an specific input pin.
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of the input pin.
    /// * `val` - Value to be setted.
    fn set_in(&mut self, idx: usize, val: bool) {
        assert!(
            idx < self.ins().len(),
            "Invalid index {} for component {} with {} inputs.",
            idx,
            self.name(),
            self.ins().len()
        );
        self.ins()[idx] = val;
    }

    /// Sets a value to an specific output pin.
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of the output pin.
    /// * `val` - Value to be setted.
    fn set_out(&mut self, idx: usize, val: bool) {
        assert!(
            idx < self.outs().len(),
            "Invalid index {} for component {} with {} inputs.",
            idx,
            self.name(),
            self.outs().len()
        );
        self.outs()[idx] = val;
    }
}
