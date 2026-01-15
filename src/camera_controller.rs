use bevy::prelude::*;

pub struct CameraControllerPlugin;
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(35.0, 20.0, 35.0).looking_at(vec3(0., 2., 0.), Vec3::Y),
    ));
}
