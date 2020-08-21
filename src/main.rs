use bevy::prelude::*;

mod plugins;

mod types;
pub use types::*;

fn main() {
    println!("Starting up");

    App::build()
        .add_default_plugins()
        // Startup
        .add_startup_system(ss_add_lights.system())
        // Systems
        // Plugins
        .add_plugin(plugins::PlayerPlugin)
        .add_plugin(plugins::DiagnosticsUIPlugin)
        .add_plugin(plugins::MainCameraPlugin)
        .add_plugin(plugins::TerrainPlugin)
        .run();
}

fn ss_add_lights(mut commands: Commands) {
    commands
        // Light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, -4.0),
            ..Default::default()
        });
}
// Resources

// @TODO: Add a `Renderable` component(whatever this translates to in Bevy)

// Plugins
