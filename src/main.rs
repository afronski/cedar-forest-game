use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod camera;
pub mod game;
pub mod game_ui;
mod helpers;
pub mod menu;
pub mod mushroom;

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
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
            camera::GlobalCameraPlugin,
            menu::MenuPlugin,
            game::GamePlugin,
            game_ui::GameUiPlugin,
            mushroom::MushroomPlugin,
        ))
        .insert_state(menu::GameState::Menu)
        .run();
}
