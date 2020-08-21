/// Size of chunk in world units
/// @TODO: Maybe this should later be expressed in tiles (and then to world units)
pub const CHUNK_WORLD_SIZE: f32 = 32.0;

/// View distance around measured in chunks
pub const VIEW_DIST_CHUNKS: i32 = 4;

// @TODO: See HMH on how to encode chunk coords in a u64?
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}
