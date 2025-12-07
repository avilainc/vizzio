//! Array implementations

pub mod primitive;
pub mod boolean;
pub mod string;
pub mod list;
pub mod r#struct;
pub mod map;
pub mod union;
pub mod builder;

pub use primitive::*;
pub use boolean::*;
pub use string::*;
pub use list::*;
pub use r#struct::*;
pub use map::*;
pub use union::*;
