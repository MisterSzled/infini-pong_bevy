use bevy::prelude::*;


#[derive(Resource, Debug, Clone)]
pub struct Level {
        pub level_data: Vec<Vec<f32>>
}

impl Level {
        pub fn new(level_data: Vec<Vec<f32>>) -> Self {
                Level { 
                        level_data
                 }
        }
}
