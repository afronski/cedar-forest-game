use bevy::prelude::*;

use crate::{Money, Player};

pub struct MushroomPlugin;

impl Plugin for MushroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_mushroom_parent)
            .add_systems(Update, (spawn_mushroom, mushroom_lifetime))
            .register_type::<Mushroom>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Mushroom {
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct MushroomParent;

fn spawn_mushroom_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), MushroomParent, Name::new("Mushroom Parent")));
}

fn spawn_mushroom(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<MushroomParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a mushroom, remaining money: ${:?}", money.0);

        let texture = asset_server.load("mushroom.png");

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..default()
                },
                Mushroom {
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
                Name::new("Mushroom"),
            ));
        });
    }
}

fn mushroom_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut mushrooms: Query<(Entity, &mut Mushroom)>,
    parent: Query<Entity, With<MushroomParent>>,
    mut money: ResMut<Money>,
) {
    let parent = parent.single();

    for (mushroom_entity, mut mushroom) in &mut mushrooms {
        mushroom.lifetime.tick(time.delta());

        if mushroom.lifetime.finished() {
            money.0 += 25.0;

            commands.entity(parent).remove_children(&[mushroom_entity]);
            commands.entity(mushroom_entity).despawn();

            info!("Mushroom sold for $15! Current Money: ${:?}", money.0);
        }
    }
}