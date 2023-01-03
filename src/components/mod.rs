pub mod component;
pub mod component_cast;
pub mod composed_component;
pub mod primitives;

pub mod prelude {
    pub use crate::components::component::{CompEvent, Component};
    pub use crate::components::component_cast::ComponentCast;
    pub use crate::components::composed_component::{
        ComposedComponent, ComposedComponentBuilder, Conn, PinAddr,
    };
    pub use crate::components::primitives::prelude::*;
    pub use crate::conn;
}
