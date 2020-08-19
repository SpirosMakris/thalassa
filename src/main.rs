use bevy::prelude::*;

mod plugins;


fn main() {
    println!("Starting up");

    App::build()
        .add_default_plugins()

        // Startup
        .add_startup_system(ss_add_camera_and_lights.system())

        // Systems

        // Plugins
        .add_plugin(plugins::PlayerPlugin)
        .add_plugin(plugins::DiagnosticsUIPlugin)
        .run();
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


// Resources



// @TODO: Add a `Renderable` component(whatever this translates to in Bevy)

// Plugins
