pub mod error;
pub mod rendering;

pub mod prelude {
    pub use crate::error::EngineError;
    pub use crate::rendering::{
        shapes::{Sprite, Square},
        Canvas, Graphics, TextLabel,
    };
}
