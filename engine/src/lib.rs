// Copyright 2022 Andre Cipriani Bandarra
//
// SPDX-License-Identifier: Apache-2.0

pub mod error;
pub mod rendering;

pub mod prelude {
    pub use crate::error::EngineError;
    pub use crate::rendering::{
        shapes::{Sprite, Square},
        Canvas, Graphics, TextLabel,
    };
}
