use bevy::prelude::*;
use clansofmesa::{assetloader::*, camera::*, player::*};

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        });

    app.add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin);

    app.add_plugin(PlayerPlugin).add_startup_system(setup);

    app.run();
}
