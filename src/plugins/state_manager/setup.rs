use bevy::prelude::*;

use crate::materials::dungeon_map::DungeonMap;
use crate::resources::levels::level::Level;
use crate::AppStates;

pub fn setup(
        mut state: ResMut<NextState<AppStates>>,

        levels: Option<Res<Level>>,
        dungeon_map: Option<Res<DungeonMap>>,
) {
        if levels.is_some() && dungeon_map.is_some() {
                state.set(AppStates::MainMenu);
        }
}
