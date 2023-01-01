pub mod component;
pub mod composed_component;
pub mod primitives;

pub mod prelude {
    pub use crate::components::component::{Component, SimEvent};
    pub use crate::components::composed_component::{
        ComposedComponent, ComposedComponentBuilder, PinAddr,
    };
}
