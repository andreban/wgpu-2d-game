// Copyright 2022 Andre Cipriani Bandarra
//
// SPDX-License-Identifier: Apache-2.0

use bomberjack::run;

fn main() {
    pollster::block_on(run());
}
