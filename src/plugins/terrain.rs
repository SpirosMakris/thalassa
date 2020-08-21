use bevy::pbr::render_graph::FORWARD_PIPELINE_HANDLE;
use bevy::prelude::*;
use bevy::render::{
    draw::Draw,
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline, RenderPipelines},
    render_graph::base::MainPass,
};

use crate::{GridPos, CHUNK_WORLD_SIZE};

const CHUNK_WIDTH_TILES: i32 = 32;
const CHUNK_HEIGHT_TILES: i32 = 32;

#[derive(Debug)]
pub struct Tile {
    pub tile_id: u32,
    pub pos: Vec2,
}

#[derive(Debug)]
pub struct Chunk {
    pub position: Vec2,
    pub tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
pub struct Map {}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TerrainType {
    Floor,
    Wall,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * CHUNK_WIDTH_TILES as usize) + x as usize
}

fn new_chunk_tiles() -> Vec<TerrainType> {
    let mut map =
        vec![TerrainType::Floor; CHUNK_WIDTH_TILES as usize * CHUNK_HEIGHT_TILES as usize];

    // Make the boundaries walls
    for x in 0..CHUNK_WIDTH_TILES {
        map[xy_idx(x, 0)] = TerrainType::Wall;
        map[xy_idx(x, CHUNK_WIDTH_TILES - 1)] = TerrainType::Wall;
    }

    for y in 0..CHUNK_HEIGHT_TILES {
        map[xy_idx(0, y)] = TerrainType::Wall;
        map[xy_idx(CHUNK_HEIGHT_TILES - 1, y)] = TerrainType::Wall;
    }

    // Randomly splat a bunch of walls
    use bracket_random::prelude::*;
    let mut rng = RandomNumberGenerator::new();

    for _i in 0..map.len() {
        let x = rng.roll_dice(1, CHUNK_WIDTH_TILES - 1);
        let y = rng.roll_dice(1, CHUNK_HEIGHT_TILES - 1);
        let idx = xy_idx(x, y);

        map[idx] = TerrainType::Wall;
    }

    map
}

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
        app.add_startup_system(ss_setup_terrain.system())
            .add_startup_system(ss_test_chunking.system());
    }
}

fn ss_setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_pos = GridPos { x: 0, y: 0 };
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

fn ss_test_chunking(resources: &mut Resources) {
    let chunky = new_chunk_tiles();
    println!("CHUNKY: {:?}", chunky);
    println!("CHUNKY len = {}", chunky.len());

    // Insert map resource
    resources.insert(chunky);
}
