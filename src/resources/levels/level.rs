use bevy::prelude::*;


#[derive(Resource, Debug, Clone)]
pub struct Level {
        pub name: String,
        pub level_data: Vec<Vec<f32>>
}

impl Level {
        pub fn new(name: String, level_data: Vec<Vec<f32>>) -> Self {
                Level { 
                        name, 
                        level_data
                 }
        }
}
