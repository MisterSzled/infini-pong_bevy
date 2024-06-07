use bevy::prelude::*;

use crate::resources::textures::sprite_sheet::SpriteSheet;
use crate::materials::dungeon_map::DungeonMap;

pub fn load(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
        // Load each sheet in turn and then attach it to the corresponding material
        let dungeon_tileset_sprite_sheet = SpriteSheet::new("tileset/Dungeon_Tileset.png", asset_server, texture_atlases);
        let dungeon_map = DungeonMap::new(dungeon_tileset_sprite_sheet);


        // Insert each material as a resource
        commands.insert_resource(dungeon_map);

        // commands.spawn(SpriteSheetBundle {
        //         transform: Transform {
        //                 scale: dungeon_map.spritesheet.scale,
        //                 ..default()
        //         },
        //         texture: dungeon_map.spritesheet.image_handle,
        //         atlas: TextureAtlas {
        //                 index: dungeon_map.top_d,
        //                 layout: dungeon_map.spritesheet.atlas_handle,
        //         },
        //         ..default()
        // });
}