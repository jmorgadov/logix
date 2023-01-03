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
    interval: u128,
    full: u128,
    val: bool,
    dirty: bool,
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
    /// let clock = Clock::new(0, 4); // Frequency 4Hz (250ms)
    /// ```
    pub fn new(frequency: f64) -> Self {
        let nano_sec_dur: u128 = (1e9 / frequency) as u128;
        Clock {
            ins: vec![],
            outs: vec![false],
            frec: frequency,
            val: false,
            interval: nano_sec_dur,
            full: nano_sec_dur * 2,
            dirty: false,
        }
    }
}

impl ComponentCast for Clock {
    fn as_clock(&self) -> Option<&Clock> {
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

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn on_event(&mut self, event: &CompEvent) {
        match event {
            CompEvent::Update(time) => {
                self.val = (time % self.full) > self.interval;
                self.dirty = self.outs[0] != self.val;
            }
            CompEvent::UpdateValues => {
                self.outs[0] = self.val;
                self.dirty = false;
            }
        }
    }
}
