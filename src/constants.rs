use bevy::prelude::Color;

pub const WINDOW_WITDH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const WINDOW_TITLE: &str = "roguelike";

pub const SPRITESHEET_COLS: usize = 5;
pub const SPRITESHEET_ROWS: usize = 1;
pub const SPRITE_TILE_WIDTH: f32 = 64.0;
pub const SPRITE_TILE_HEIGHT: f32 = 64.0;

pub const SPRITE_IDX_GRASS: usize = 0;
pub const SPRITE_IDX_GRASS_WITH_FLOWER: usize = 1;
pub const SPRITE_IDX_GRASS_WITH_STONE: usize = 2;
pub const SPRITE_IDX_PLAYER: usize = 3;
pub const SPRITE_IDX_RABBIT: usize = 4;

pub const MAP_WIDTH: usize = 50;
pub const MAP_HEIGHT: usize = 50;

pub const Z_INDEX_TILE: f32 = 0.0;

pub const UI_TEXT_TURN_COLOR: Color = Color::BLACK;
pub const UI_TEXT_TURN_SIZE: f32 = 20.0;

pub const PERLIN_NOISE_SCALE: f64 = 0.1;
