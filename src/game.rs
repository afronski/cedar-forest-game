use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::ecs::reflect::{ReflectComponent, ReflectResource};
use bevy::input::ButtonInput;
use bevy::prelude::{Camera2dBundle, Commands, Component, default, KeyCode, Query, Reflect, Res, Resource, SpriteBundle, Time, Transform};
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::menu;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

pub fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>, configured_speed: Res<menu::SpeedConfiguration>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: ((*configured_speed) as u8) as f32 },
        Name::new("Player"),
    ));
}

pub fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= movement_amount;
        }
    }
}
