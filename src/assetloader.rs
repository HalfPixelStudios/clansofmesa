use bevy::prelude::*;

const ATLAS_WIDTH: usize = 203;
const ATLAS_HEIGHT: usize = 169;

pub struct AssetSheet(pub Handle<TextureAtlas>);

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets);
    }
}

pub fn load_assets(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image: Handle<Image> = assets.load("tiles.png");

    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::new(16.0, 16.0),
        ATLAS_WIDTH,
        ATLAS_HEIGHT,
        Vec2::splat(1.0),
    );
    let atlas_handle = texture_atlases.add(atlas);
    cmd.insert_resource(AssetSheet(atlas_handle));
}
