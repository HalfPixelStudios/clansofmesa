use bevy::prelude::*;
use clansofmesa::{assetloader::*, camera::*, tower::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(TowerPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
