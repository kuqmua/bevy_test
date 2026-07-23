use bevy::prelude::*;
mod player;
mod camera;
mod world;
use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, WorldPlugin))
        .run()
}
