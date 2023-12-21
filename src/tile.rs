use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Component)]
pub enum TileType {
    Grass,
    GrassWithFlower,
    GrassWithStone,
}

impl TileType {
    pub const fn to_sprite_idx(tile_type: &TileType) -> usize {
        match tile_type {
            TileType::Grass => SPRITE_IDX_GRASS,
            TileType::GrassWithFlower => SPRITE_IDX_GRASS_WITH_FLOWER,
            TileType::GrassWithStone => SPRITE_IDX_GRASS_WITH_STONE,
        }
    }

    pub fn is_walkable(self) -> bool {
        match self {
            TileType::Grass => true,
            TileType::GrassWithFlower => true,
            TileType::GrassWithStone => false,
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub r#type: TileType,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
}
