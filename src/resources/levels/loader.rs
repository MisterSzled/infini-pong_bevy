use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::resources::levels::level::Level;

const PREFIX: &str = "./assets/map/";

pub fn load(
        mut commands: Commands, 
        // asset_server: Res<AssetServer>
) 
{
        let file_name = "01.txt";
        let path = format!("{}{}", PREFIX, file_name);
        // let map_name = path.clone();
        let file = match File::open(path) {
                Ok(file) => file,
                Err(err) => panic!("Can't open map file {}: {}", file_name, err.to_string()),
        };

        let reader = BufReader::new(file);

        let mut map: Vec<Vec<f32>> = Vec::new();

        for (_, line) in reader.lines().enumerate() {
                let str_line = line.unwrap();
                let str_numbers = str_line.split(" ");

                let mut row: Vec<f32> = Vec::new();

                for (_, str_number) in str_numbers.enumerate() {
                        let number: f32 = str_number.parse::<f32>().unwrap();

                        row.push(number);
                }
                map.push(row);
        }

        let level = Level::new(map);

        commands.insert_resource(level);
}
