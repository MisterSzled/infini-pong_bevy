use bevy::prelude::*;

use crate::resources::levels::level::Level;
use crate::materials::dungeon_map::DungeonMap;

pub fn setup(
        mut commands: Commands, 
        asset_server: Res<AssetServer>,
        levels: Res<Level>,
        dungeon_map: Res<DungeonMap>
) {
        println!("!")
}