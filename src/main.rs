use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*
};

fn main() {
    println!("Starting up");

    App::build()
        .add_default_plugins()
        // Startup
        .add_startup_system(hello_world_system.system())
        .add_startup_system(ss_add_player.system())
        .add_startup_system(ss_add_camera_and_lights.system())
        // .add_startup_system(ss_player_state.system())
        .init_resource::<PlayerMotion>()
        // Systems
        .add_system(s_keyboard_input_system.system())
        .add_system(s_update_player.system())
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
        })
        .with(Player)
        .with(Position { x: 0.0, y: 0.0 });
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
                Vec3::new(50.0, 100.0, 100.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

fn s_keyboard_input_system(mut commands: Commands, mut player_motion: ResMut<PlayerMotion>, keyboard_input: Res<Input<KeyCode>>) {
    // Left-Right axis
    let left_pressed = keyboard_input.pressed(KeyCode::A);
    let right_pressed = keyboard_input.pressed(KeyCode::D);

    if left_pressed {
        player_motion.right_axis = -1.0;
    } else if right_pressed {
        player_motion.right_axis = 1.0;        
    } else {
        player_motion.right_axis = 0.0;
    }

    // Up-Down axis
    let up_pressed = keyboard_input.pressed(KeyCode::W);
    let down_presssed = keyboard_input.pressed(KeyCode::S);

    if up_pressed {
        player_motion.up_axis = -1.0;
    } else if down_presssed {
        player_motion.up_axis = 1.0;
    } else {
        player_motion.up_axis = 0.0;
    }

}

fn s_update_player(player_motion: Res<PlayerMotion>, mut query: Query<(&Player, &mut Translation)>) {

    for (_player, mut translation) in &mut query.iter() {
        
        translation.0 += Vec3::new(player_motion.right_axis, 0.0, player_motion.up_axis);
    }
}


// Resources
#[derive(Default)]
struct PlayerMotion {
    right_axis: f32,
    up_axis: f32
}

// Components

struct Position {
    x: f32,
    y: f32,
}

struct Player;

// @TODO: Add a `Renderable` component(whatever this translates to in Bevy)