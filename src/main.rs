use bevy::prelude::*;
use clansofmesa::player;

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugin(player::PlayerPlugin)
        .add_startup_system(setup);

    app.run();
}
