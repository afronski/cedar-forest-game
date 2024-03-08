mod game;
mod game_ui;
mod menu;
mod mushroom;
mod camera;

use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::camera::GlobalCameraPlugin;
use crate::game::GamePlugin;
use crate::game_ui::GameUiPlugin;
use crate::menu::MenuPlugin;
use crate::mushroom::MushroomPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cedar Forest".into(),
                        resolution: (800.0, 600.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            (
                WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
                GlobalCameraPlugin,
                MenuPlugin,
                GamePlugin,
                GameUiPlugin,
                MushroomPlugin
            )
        )
        .insert_state(menu::GameState::Menu)
        .run();
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
