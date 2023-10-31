#![warn(clippy::all)]

// Modules

mod map;
mod map_builder;
mod player;

// Prelude

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 40;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

// Structs

struct State {
    map: Map,
    player: Player,
}

// Implementations

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

// Functions

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Test")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
