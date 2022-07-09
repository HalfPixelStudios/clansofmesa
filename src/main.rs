use bevy::prelude::*;
use clansofmesa::{assetloader::*, camera::*, networking::*, player::*};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        });

    app.add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin);

    app.add_plugin(PlayerPlugin);

    app.run();
}
