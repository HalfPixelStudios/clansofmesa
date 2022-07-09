use bevy::prelude::*;

const ATLAS_WIDTH: usize = 896;
const ATLAS_HEIGHT: usize = 448;
pub struct BlockSheet(pub Handle<TextureAtlas>);

pub fn load_blocks(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let image: Handle<Image> = assets.load("tilesheet.png");

    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::new(64.0, 64.0),
        ATLAS_WIDTH,
        ATLAS_HEIGHT,
    );
    let atlas_handle = texture_atlases.add(atlas);
    cmd.insert_resource(BlockSheet(atlas_handle));
}
pub struct AssetLoadPlugin;
impl Plugin for AssetLoadPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_blocks);
    }
}
