use bevy::prelude::*;

fn main() {
    println!("Starting up");

    App::build()
        .add_default_plugins()
        .add_startup_system(hello_world_system.system())
        .add_startup_system(ss_add_player.system())
        .add_startup_system(ss_add_camera_and_lights.system())
        .run();
}

fn hello_world_system() {
    println!("Hello world!");
}

fn ss_add_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });

    commands
        .spawn(PbrComponents {
            mesh: cube_handle,
            material: cube_material_handle,
            translation: Translation::new(0.0, 0.0, 1.0),
            ..Default::default()
        });
}

fn ss_add_camera_and_lights(mut commands: Commands) {
    commands
        // Light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, -4.0),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(5.0, 10.0, 10.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}
