use bevy::pbr::render_graph::FORWARD_PIPELINE_HANDLE;
use bevy::prelude::*;
use bevy::render::{
    draw::Draw,
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline, RenderPipelines},
    render_graph::base::MainPass,
};

use crate::{GridPos, CHUNK_WORLD_SIZE};

/// A component bundle for a TerrainChunk
/// Based on PBRComponents
#[derive(Bundle)]
struct TerrainChunkComponents {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
    pub grid_pos: GridPos,
}

impl Default for TerrainChunkComponents {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                FORWARD_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 2,
                            binding: 0,
                        },
                        // StandardMaterial_albedo
                        DynamicBinding {
                            bind_group: 3,
                            binding: 0,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            mesh: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
            grid_pos: Default::default(),
        }
    }
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(ss_setup_terrain.system());
    }
}

fn ss_setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(PbrComponents {
    //     mesh: meshes.add(Mesh::from(shape::Plane {
    //         size: CHUNK_WORLD_SIZE,
    //     })),
    //     material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
    //     ..Default::default()
    // });
    let grid_pos = GridPos { x: 2, y: 1 };
    let translation = Translation::new(
        grid_pos.x as f32 * CHUNK_WORLD_SIZE,
        0.0,
        grid_pos.y as f32 * CHUNK_WORLD_SIZE,
    );

    commands.spawn(TerrainChunkComponents {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: CHUNK_WORLD_SIZE,
        })),
        material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        grid_pos,
        translation,
        ..Default::default()
    });
}
