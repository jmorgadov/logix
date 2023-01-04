use logix::prelude::*;

pub struct UpdateTimeVisitor;

impl UpdateTimeVisitor {
    pub fn visit_clock(time: u128, comp: &mut Clock) -> bool {
        let val = (time % comp.full_cycle) > comp.interval;
        let dirty = comp.outs[0] != val;
        comp.outs[0] = val;
        dirty
    }

    pub fn visit_composed(time: u128, comp: &mut ComposedComponent) -> bool {
        let mut dirty = false;
        for sub_comp in comp.components.iter_mut() {
            if let Some(clock) = sub_comp.as_clock_mut() {
                dirty |= UpdateTimeVisitor::visit_clock(time, clock);
            }
        }
        dirty
    }
}
