pub mod input;
pub mod rendering;

pub mod prelude {
    pub use crate::rendering::{
        Canvas,
        Graphics,
        shapes::{
            Sprite,
            Square
        }
    };
}