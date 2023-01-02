use std::fmt::Debug;

use crate::serialize::JSONSerialize;

/// Represents a component event
///
/// An event is notified via the `on_event` function of the `Component` trait.
pub enum CompEvent {
    /// Notifies an update in time.
    Update(u128),
    /// Notifies the component to update its output values.
    UpdateValues,
}

/// Basic trait that describes a component.
///
/// # Implementation details
///
/// All components that implement the `Component` trait are consider primitives,
/// except for the `ComposedComponent`. It is a special implementation of this
/// trait which can hold several components (even composed ones) and connect them
/// to make a new bigger component.
pub trait Component: Debug + JSONSerialize {
    /// Returns the id of the component.
    fn id(&self) -> u32;

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

    /// Returns a bool indicating if the component is dirty, by default false.
    ///
    /// When a component is dirty, all the components updates their output values
    /// according their inputs. For the composed components the update is done
    /// taking into account the dependencies between the components according the
    /// connections.
    fn is_dirty(&self) -> bool {
        false
    }

    /// This function runs every time an event occurs in the simulation,
    /// like updating the time (`Update(time)`), or updating the values
    /// `UpdateValues`.
    ///
    /// # Arguments
    ///
    /// * `event` - An enum instance of `SimEvent` representing the event type.
    fn on_event(&mut self, _event: &CompEvent) {}
}
