use std::ops::{Add, AddAssign, Mul};

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_enhanced_input::prelude::*;

#[derive(Component)]
struct CameraController;

#[derive(InputAction)]
#[action_output(Vec2)]
struct CameraLook;

#[derive(InputAction)]
#[action_output(Vec2)]
struct CameraMove;

#[derive(InputAction)]
#[action_output(bool)]
struct GrabCursor;

#[derive(InputAction)]
#[action_output(bool)]
struct ReleaseCursor;

pub struct CameraControllerPlugin;
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<CameraController>()
            .add_systems(Startup, setup_camera)
            .add_observer(update_camera_rotation)
            .add_observer(update_camera_position)
            .add_observer(grab_cursor)
            .add_observer(release_cursor);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(35.0, 20.0, 35.0).looking_at(vec3(0., 2., 0.), Vec3::Y),
        CameraController,
        actions!(
            CameraController[
                (Action::<CameraLook>::new(), bindings![(Binding::mouse_motion(), Negate::all())]),
                (Action::<CameraMove>::new(), Bindings::spawn(Cardinal::wasd_keys())),
                (Action::<GrabCursor>::new(), bindings![MouseButton::Left]),
                (Action::<ReleaseCursor>::new(), bindings![KeyCode::KeyG])
            ]
        ),
    ));
}

fn update_camera_rotation(
    camera_look: On<Fire<CameraLook>>,
    mut camera: Single<&mut Transform, With<CameraController>>,
    cursor_options: Single<&mut CursorOptions>,
    time: Res<Time>,
) {
    if cursor_options.grab_mode != CursorGrabMode::Confined {
        return;
    }

    let delta_secs = time.delta_secs();
    let sensitivity = 5.;
    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
    yaw += camera_look.value.x.to_radians() * delta_secs * sensitivity;
    pitch += camera_look.value.y.to_radians() * delta_secs * sensitivity;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
}

fn update_camera_position(
    camera_move: On<Fire<CameraMove>>,
    mut camera: Single<&mut Transform, With<CameraController>>,
    cursor_options: Single<&mut CursorOptions>,
    time: Res<Time>,
) {
    if cursor_options.grab_mode != CursorGrabMode::Confined {
        return;
    }

    let movement_speed = 20.;
    let camera_front = camera.forward().mul(camera_move.value.y);
    let camera_right = camera.right().mul(camera_move.value.x);
    let movement = camera_front
        .add(camera_right)
        .normalize()
        .mul(movement_speed * time.delta_secs());
    camera.translation.add_assign(movement);
}

fn grab_cursor(_: On<Complete<GrabCursor>>, cursor_options: Single<&mut CursorOptions>) {
    update_cursor(cursor_options, true);
}

fn release_cursor(_: On<Complete<ReleaseCursor>>, cursor_options: Single<&mut CursorOptions>) {
    update_cursor(cursor_options, false);
}

fn update_cursor(mut cursor_options: Single<&mut CursorOptions>, grab: bool) {
    cursor_options.grab_mode = if grab {
        CursorGrabMode::Confined
    } else {
        CursorGrabMode::None
    };
    cursor_options.visible = !grab;
}
