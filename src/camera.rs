use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct GlobalCameraPlugin;

impl Plugin for GlobalCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, global_camera_setup);
    }
}

fn global_camera_setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);
}
