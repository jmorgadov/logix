pub mod component;
pub mod composed_component;
pub mod primitives;

pub mod prelude {
    pub use crate::components::component::{Component, CompEvent};
    pub use crate::components::composed_component::{
        ComposedComponent, ComposedComponentBuilder, PinAddr,
    };
}
