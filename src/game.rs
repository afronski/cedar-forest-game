use crate::menu;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(menu::GameState::Game), game_setup)
            .add_systems(Update, character_movement)
            .register_type::<Money>()
            .register_type::<Player>()
            .insert_resource(Money(100.0));
    }
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    configured_speed: Res<menu::SpeedConfiguration>,
) {
    let texture = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player {
            speed: ((*configured_speed) as u8) as f32,
        },
        Name::new("Player"),
    ));
}

fn character_movement(
    mut app_exit_events: EventWriter<AppExit>,
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
        if input.pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit);
        }
    }
}
