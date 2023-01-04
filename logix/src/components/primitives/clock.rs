use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents a clock.
///
/// This component updates its value between one and zero (true/false) in a specific
/// frequency.
#[derive(Debug)]
pub struct Clock {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,

    pub frec: f64,
    pub interval: u128,
    pub full_cycle: u128,
}

impl Clock {
    /// Creates a new `Clock` component given an id and the update frequency.
    ///
    /// # Arguments
    ///
    /// * `frequency` - Float that represent the update frequency in Hertz.
    ///
    /// # Example
    ///
    /// ```
    /// # use logix::prelude::Clock;
    /// #
    /// let clock = Clock::new(4.0); // Frequency 4Hz (250ms)
    /// ```
    pub fn new(frequency: f64) -> Self {
        let nano_sec_dur: u128 = (1e9 / frequency) as u128;
        Clock {
            ins: vec![],
            outs: vec![false],
            frec: frequency,
            interval: nano_sec_dur,
            full_cycle: nano_sec_dur * 2,
        }
    }
}

impl ComponentCast for Clock {
    fn as_clock(&self) -> Option<&Clock> {
        Some(self)
    }
    fn as_clock_mut(&mut self) -> Option<&mut Clock> {
        Some(self)
    }
}

impl Component for Clock {
    fn name(&self) -> String {
        Primitive::Clock.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}
