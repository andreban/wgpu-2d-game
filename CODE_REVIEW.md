# Code Review

## High Priority

- [x] **1. Unchecked `.unwrap()` calls** — `engine/src/rendering/mod.rs:61,68`, `engine/src/rendering/pipelines/sprite.rs:91`
  The adapter/device initialization and texture loading use `.unwrap()`. These should propagate errors with `?` rather than panicking on GPU unavailability or missing assets.

- [x] **2. Animation index out-of-bounds** — `bomberjack/src/game/mod.rs:31-43`
  `self.frames[self.current_frame]` will panic if `frames` is empty. Use `.get()` or add a debug assertion.

## Medium Priority

- [ ] **3. Texture hardcoded in the engine** — `engine/src/rendering/pipelines/sprite.rs:89`
  The engine library directly references the game's texture, destroying its reusability. Texture should be passed as a parameter to `SpritePipeline::new()`.

- [ ] **4. Hardcoded camera dimensions** — `engine/src/rendering/pipelines/sprite.rs:137`, `engine/src/rendering/pipelines/square.rs:92`
  `Camera2d::new(600.0, 650.0)` is hardcoded inside pipeline constructors. These should come from `SurfaceConfiguration` or be constructor parameters.

- [ ] **5. Instance buffer overflow** — `engine/src/rendering/pipelines/sprite.rs:266-272`
  No check that `sprites.len() <= MAX_INSTANCES`. Writing past the buffer limit is silent corruption. Add a bounds check before `write_buffer`.

- [ ] **6. Text buffer recreated every frame** — `engine/src/rendering/mod.rs:181-194`
  A new `glyphon::Buffer` is allocated on every render call. This should be cached and only updated when the score changes.

- [ ] **7. Floating-point equality comparison** — `bomberjack/src/game/mod.rs:211,222`
  `self.jack.position.y == original_y` is unreliable due to `f32` accumulation errors. Use an epsilon comparison.

- [ ] **8. Silent render failure** — `engine/src/rendering/mod.rs:160-165`
  Surface texture acquisition failures are silently ignored (`_ => return`). At minimum, log the error.

## Low Priority

- [ ] **9. `Canvas::done()` is redundant** — `engine/src/rendering/mod.rs:156`
  `drop(self)` inside `done()` is a no-op since the caller could just drop it. The implicit rendering-on-drop pattern should at least be clearly documented.

- [ ] **10. `Bomb::disarmed` field** — `bomberjack/src/game/bomb.rs:17,39`
  This field is initialized but never read. Remove it or implement the mechanic.

- [ ] **11. Magic numbers in collision detection** — `bomberjack/src/game/mod.rs:259`
  The `4.0` platform tolerance should be a named constant: `const PLATFORM_COLLISION_TOLERANCE: f32 = 4.0;`

- [ ] **12. `InputState` missing `#[derive(Default)]`** — `bomberjack/src/input.rs`
  The `new()` method just zero-initializes bools. Replace with `#[derive(Default)]`.

- [ ] **13. Empty `engine/src/input/mod.rs`**
  The module exists but is completely empty. Either implement input abstractions there or remove the module.

- [ ] **14. `&mut Device` / `&mut Queue` parameters** — `engine/src/rendering/pipelines/sprite.rs:85-87`
  wgpu `Device` and `Queue` use interior mutability. Shared references (`&Device`, `&Queue`) are sufficient and would improve ergonomics.
