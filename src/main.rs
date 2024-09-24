use bevy::
    prelude::*
;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_obj::ObjPlugin;

mod camera;
mod player;
mod world;
mod cursor;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use cursor::CursorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            PlayerPlugin,
            WorldPlugin,
            ObjPlugin,
			CursorPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
