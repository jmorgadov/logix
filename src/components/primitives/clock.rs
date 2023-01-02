use crate::{
    components::component::{Component, SimEvent},
    serialize::JSONSerialize,
};

use super::primitive::Primitive;

/// Represents a clock.
///
/// This component updates its value between one and zero (true/false) in a specific
/// frequency.
#[derive(Debug)]
pub struct Clock {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,

    frec: f64,
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
    /// * `id` - Integer that represents the component id.
    /// * `frequency` - Float that represent the update frequency in Hertz.
    ///
    /// # Example
    ///
    /// ```
    /// let clock = Clock::new(0, 4); // Frequency 4Hz (250ms)
    /// ```
    pub fn new(id: u32, frequency: f64) -> Self {
        let nano_sec_dur: u128 = (1e9 / frequency) as u128;
        Clock {
            id,
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

impl JSONSerialize for Clock {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": Primitive::Clock.to_string(),
            "frec": self.frec,
        })
    }

    fn from_json(json: &serde_json::Value) -> Self
    where
        Self: Sized,
    {
        Clock::new(
            json["id"].as_u64().unwrap() as u32,
            json["frec"].as_f64().unwrap(),
        )
    }
}

impl Component for Clock {
    fn id(&self) -> u32 {
        self.id
    }

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

    fn on_event(&mut self, event: &SimEvent) {
        match event {
            SimEvent::Update(time) => {
                self.val = (time % self.full) > self.interval;
                self.dirty = self.outs[0] != self.val;
            }
            SimEvent::UpdateValues => {
                self.outs[0] = self.val;
                self.dirty = false;
            }
        }
    }
}
