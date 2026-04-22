// Copyright 2022 Andre Cipriani Bandarra
//
// SPDX-License-Identifier: Apache-2.0

use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};

pub struct TextLabel {
    pub(super) buffer: Buffer,
}

impl TextLabel {
    pub fn new(font_system: &mut FontSystem, text: &str) -> Self {
        let mut buffer = Buffer::new(font_system, Metrics::new(30.0, 42.0));
        buffer.set_text(
            font_system,
            text,
            &Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(font_system, false);
        Self { buffer }
    }

    pub fn set_text(&mut self, font_system: &mut FontSystem, text: &str) {
        self.buffer.set_text(
            font_system,
            text,
            &Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
            None,
        );
        self.buffer.shape_until_scroll(font_system, false);
    }
}
