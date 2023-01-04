use std::time::Instant;

use logix::prelude::*;

use crate::visitors::{
    update_time_visitor::UpdateTimeVisitor, update_values_visitor::UpdateValuesVisitor,
};

/// Simulation.
pub struct Simulation {
    comp: ComposedComponent,
    running: bool,
}

impl Simulation {
    /// Creates a new simulation given the main component.
    ///
    /// # Arguments
    ///
    /// * `comp` - A box containing the main component.
    pub fn new(comp: ComposedComponent) -> Self {
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
            let dirty = UpdateTimeVisitor::visit_composed(time, &mut self.comp);
            if dirty {
                UpdateValuesVisitor::visit_composed(&mut self.comp);
                println!("{:?}", self.comp.outs());
            }
        }
    }
}
