use bevy_ecs_tilemap::tiles::{TileBundle, TileColor, TilePos, TileTextureIndex};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::mouse::CursorPos;

pub struct TilesetPlugin;
impl Plugin for TilesetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, highlight_tile_labels)
            .add_plugins(TilemapPlugin);
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle: Handle<Image> = asset_server.load("16x16sprites.png");
    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    fill_tilemap(
        TileTextureIndex(80),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

   #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("16x16sprites.png")),
            tile_size,
            ..Default::default()
        });
    }
}


#[derive(Component)]
struct HighlightedLabel;


fn highlight_tile_labels(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    highlighted_tiles_q: Query<Entity, With<HighlightedLabel>>,
) {
    
    // Un-highlight any previously highlighted tile labels.
    for highlighted_tile_entity in highlighted_tiles_q.iter() {
        commands
            .entity(highlighted_tile_entity)
            .remove::<HighlightedLabel>()
            .insert(TileColor::default());

    }

    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
        // Grab the cursor position from the `Res<CursorPos>`
        let cursor_pos: Vec2 = cursor_pos.0;
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            // Highlight the relevant tile's label
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                info!("Entity {:?} is", cursor_in_map_pos);
                commands.entity(tile_entity)
                    .insert(TileColor(Color::srgba(10.0, 1.0, 0.5, 0.6)))
                    .insert(HighlightedLabel);
            }
        }
    }
}