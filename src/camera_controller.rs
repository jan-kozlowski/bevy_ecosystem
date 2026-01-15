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
            .add_observer(update_camera_view)
            .add_observer(grab_cursor)
            .add_observer(release_cursor);

        println!("Camera controller plugin ready!");
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
                (Action::<GrabCursor>::new(), bindings![MouseButton::Left]),
                (Action::<ReleaseCursor>::new(), bindings![KeyCode::KeyG])
            ]
        ),
    ));
}

fn update_camera_view(
    camera_look: On<Fire<CameraLook>>,
    mut camera: Single<&mut Transform, With<CameraController>>,
    cursor_options: Single<&mut CursorOptions>,
) {
    if cursor_options.grab_mode != CursorGrabMode::Confined {
        return;
    }

    let val = camera_look.value;
    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
    yaw += val.x.to_radians();
    pitch += val.y.to_radians();
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
    println!("camera look: {:?}", val);
}

fn grab_cursor(grab_cursor: On<Complete<GrabCursor>>, cursor_options: Single<&mut CursorOptions>) {
    let val = grab_cursor.value;
    println!("grabbing cursor: {:?}", val);
    update_cursor(cursor_options, true);
}

fn release_cursor(
    grab_cursor: On<Complete<ReleaseCursor>>,
    cursor_options: Single<&mut CursorOptions>,
) {
    let val = grab_cursor.value;
    println!("releasing cursor: {:?}", val);
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
