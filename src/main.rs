use bevy::prelude::*;

mod map;
mod dice;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
