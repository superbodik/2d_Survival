use rand::Rng;
use noise::{NoiseFn, Perlin};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BiomeType {
    Summer,
    Winter,
}

impl BiomeType {
    pub fn get_temperature(&self) -> f32 {
        match self {
            BiomeType::Summer => 0.7,
            BiomeType::Winter => 0.3,
        }
    }

    pub fn get_humidity(&self) -> f32 {
        match self {
            BiomeType::Summer => 0.5,
            BiomeType::Winter => 0.3,
        }
    }
}

pub fn determine_biome(temperature: f32, _humidity: f32) -> BiomeType {
    if temperature > 0.5 {
        BiomeType::Summer
    } else {
        BiomeType::Winter
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Grass { rotation: f32 },
    Water,
    Dirt,
    Road,
    BiomeBorder { from: BiomeType, to: BiomeType },
}

#[derive(Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: (i32, i32),
    pub biome: BiomeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition(pub i32, pub i32);

const CHUNK_SIZE: i32 = 16;

pub fn generate_chunk(chunk_pos: ChunkPosition, seed: u64) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut rng = rand::thread_rng();
    
    let biome = if noise_2d(chunk_pos.0 as f32 * 0.1, chunk_pos.1 as f32 * 0.1, seed) > 0.5 {
        BiomeType::Summer
    } else {
        BiomeType::Winter
    };

    for local_y in 0..CHUNK_SIZE {
        for local_x in 0..CHUNK_SIZE {
            let world_x = chunk_pos.0 * CHUNK_SIZE + local_x;
            let world_y = chunk_pos.1 * CHUNK_SIZE + local_y;

            let tile_type = if world_x % 32 == 0 {
                TileType::Road
            } else {
                let chance: f32 = rng.gen();
                match biome {
                    BiomeType::Summer => {
                        if chance < 0.05 {
                            TileType::Water
                        } else if chance < 0.3 {
                            TileType::Dirt
                        } else {
                            TileType::Grass { rotation: 0.0 }
                        }
                    }
                    BiomeType::Winter => {
                        if chance < 0.05 {
                            TileType::Water
                        } else {
                            TileType::Grass { rotation: 0.0 }
                        }
                    }
                }
            };

            tiles.push(Tile {
                tile_type,
                position: (world_x, world_y),
                biome,
            });
        }
    }

    tiles
}

fn noise_2d(x: f32, y: f32, seed: u64) -> f32 {
    (x.sin() + y.cos() + seed as f32).sin() * 0.5 + 0.5
}
