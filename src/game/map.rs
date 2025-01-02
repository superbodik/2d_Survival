use bevy::prelude::*;
use std::collections::HashMap;
use crate::game::generate_map::{ChunkPosition, generate_chunk, BiomeType, TileType};

#[derive(Resource)]
pub struct MapState {
    loaded_chunks: HashMap<ChunkPosition, Entity>,
    seed: u64,
}

const CHUNK_SIZE: i32 = 16;
const RENDER_DISTANCE: i32 = 2;

pub fn setup_map(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(MapState {
        loaded_chunks: HashMap::new(),
        seed: rand::random(),
    });
}

pub fn update_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<crate::game::player::Player>>,
    mut map_state: ResMut<MapState>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_chunk = ChunkPosition(
            (player_transform.translation.x / (CHUNK_SIZE as f32 * 32.0)).floor() as i32,
            (player_transform.translation.y / (CHUNK_SIZE as f32 * 32.0)).floor() as i32,
        );

        // Определяем какие чанки должны быть загружены
        let mut chunks_to_load = Vec::new();
        for y in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let chunk_pos = ChunkPosition(
                    player_chunk.0 + x,
                    player_chunk.1 + y,
                );
                chunks_to_load.push(chunk_pos);
            }
        }

        // Удаляем дальние чанки
        let chunks_to_remove: Vec<_> = map_state.loaded_chunks
            .keys()
            .filter(|pos| !chunks_to_load.contains(pos))
            .copied()
            .collect();

        for pos in chunks_to_remove {
            if let Some(entity) = map_state.loaded_chunks.remove(&pos) {
                commands.entity(entity).despawn_recursive();
            }
        }

        // Загружаем новые чанки
        for chunk_pos in chunks_to_load {
            if !map_state.loaded_chunks.contains_key(&chunk_pos) {
                let chunk_entity = spawn_chunk(&mut commands, &asset_server, chunk_pos, map_state.seed);
                map_state.loaded_chunks.insert(chunk_pos, chunk_entity);
                println!("Загружен чанк: {:?}", chunk_pos); // Отладочный вывод
            }
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    chunk_pos: ChunkPosition,
    seed: u64,
) -> Entity {
    let chunk = commands.spawn(SpatialBundle::default()).id();
    let tiles = generate_chunk(chunk_pos, seed);

    for tile in tiles {
        let (texture_path, rotation) = match tile.tile_type {
            TileType::Grass { rotation } => {
                match tile.biome {
                    BiomeType::Summer => (format!("summer/grass_{}.png", rand::random::<u8>() % 3), rotation),
                    BiomeType::Winter => (format!("winter/grass_{}.png", rand::random::<u8>() % 3), rotation),
                }
            },
            TileType::Water => {
                match tile.biome {
                    BiomeType::Summer => ("summer/water.png".to_string(), 0.0),
                    BiomeType::Winter => ("winter/water.png".to_string(), 0.0),
                }
            },
            TileType::Dirt => {
                match tile.biome {
                    BiomeType::Summer => ("summer/dirt.png".to_string(), 0.0),
                    BiomeType::Winter => ("winter/dirt.png".to_string(), 0.0),
                }
            },
            TileType::Road => ("common/road.png".to_string(), 0.0),
            TileType::BiomeBorder { from, to } => {
                (format!("borders/{}_to_{}.png", 
                    if from == BiomeType::Summer { "summer" } else { "winter" },
                    if to == BiomeType::Summer { "summer" } else { "winter" }
                ), 0.0)
            },
        };

        commands.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    tile.position.0 as f32 * 32.0,
                    tile.position.1 as f32 * 32.0,
                    0.0,
                ),
                rotation: Quat::from_rotation_z(rotation.to_radians()),
                scale: Vec3::new(32.0, 32.0, 1.0),
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture: asset_server.load(&texture_path),
            ..Default::default()
        }).set_parent(chunk);
    }

    chunk
}
