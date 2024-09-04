use bevy::prelude::*;

mod debug;
mod camera;
mod tileset;
mod mouse;

use mouse::{CursorPos, CursorPlugin};
use debug::DebugPlugin;
use tileset::TilesetPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("My proto"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<CursorPos>()
        .add_plugins(TilesetPlugin)
        .add_plugins(CursorPlugin)
        .add_systems(Update, camera::movement)
        .run();
}