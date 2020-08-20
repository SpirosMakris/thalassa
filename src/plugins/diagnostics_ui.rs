use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct DiagnosticsUIPlugin;

impl Plugin for DiagnosticsUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(ss_setup_diagnostics_ui.system())
            .add_system(s_update_diagnostics_ui.system());
    }
}

fn ss_setup_diagnostics_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap();

    commands
        // UI camera
        .spawn(UiCameraComponents::default())
        // text
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },

            text: Text {
                value: "FPS:".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        });
}

// @TODO: Add marker component so we don't update any other text components
fn s_update_diagnostics_ui(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
