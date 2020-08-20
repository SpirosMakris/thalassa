use bevy::prelude::*;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(ss_setup_main_camera.system());
    }
}

fn ss_setup_main_camera(mut commands: Commands) {
    commands.spawn(Camera3dComponents {
        transform: Transform::new_sync_disabled(Mat4::face_toward(
            Vec3::new(50.0, 100.0, 100.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )),
        ..Default::default()
    });
}
