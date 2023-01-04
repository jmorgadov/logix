use std::time::Instant;

use crate::components::prelude::*;

/// Simulation.
pub struct Simulation {
    comp: Box<dyn Component>,
    running: bool,
}

impl Simulation {
    /// Creates a new simulation given the main component.
    ///
    /// # Arguments
    ///
    /// * `comp` - A box containing the main component.
    pub fn new(comp: Box<dyn Component>) -> Self {
        Simulation {
            comp,
            running: false,
        }
    }

    /// Starts the simulation.
    pub fn start(&mut self) {
        self.running = true;

        let start = Instant::now();
        while self.running {
            let time = start.elapsed().as_nanos();
            self.comp.on_event(&CompEvent::Update(time));
            if self.comp.is_dirty() {
                self.comp.on_event(&CompEvent::UpdateValues);
                println!("{:?}", self.comp.outs());
            }
        }
    }
}
