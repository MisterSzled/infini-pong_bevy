use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct SpriteSheet {
        pub image_handle: Handle<Image>,
        pub atlas_handle: Handle<TextureAtlasLayout>,
        pub scale: Vec3,
        pub grid_size: (f32, f32)
}

impl SpriteSheet {
        pub fn new(
                path: &str,
                asset_server: Res<AssetServer>,
                mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
        ) -> Self {
                let image_handle: Handle<Image> = asset_server.load(path.to_string());
                let texture_atlas =
                        TextureAtlasLayout::from_grid(
                                Vec2::new(16.0, 16.0), 
                                10, 
                                10, 
                                None, 
                                None
                        );
                let atlas_handle = texture_atlases.add(texture_atlas);

                SpriteSheet {
                        image_handle,
                        atlas_handle,
                        scale: Vec3::splat(3.),
                        grid_size: (16., 16.)
                }
        }
}
