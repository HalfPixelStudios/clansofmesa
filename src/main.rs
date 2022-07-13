use bevy::prelude::*;
use bevy_bobs::physics_2d::PhysicsPlugin;
use bevy_ggrs::*;
use clansofmesa::{
    app_state::*,
    assetloader::*,
    camera::*,
    enemy::{
        ai::{boid_ai_system, dumb_ai_system},
        EnemyPlugin,
    },
    game::*,
    input::*,
    layers::Layers,
    map::*,
    networking::*,
    player::place::*,
    structure::*,
    ui::UIPlugin,
};

pub enum Mode {
    Building,
    Deploying,
    Camera,
}

fn main() {
    let mut app = App::new();

    // networked systems
    GGRSPlugin::<GGRSConfig>::new()
        .with_input_system(input_system)
        .with_rollback_schedule(
            Schedule::default().with_stage(
                "ROLLBACK_STAGE",
                SystemStage::parallel()
                    .with_system(change_mode)
                    .with_system(dumb_ai_system)
                    .with_system(boid_ai_system)
                    .with_system(place_structure)
                    .with_system(place_enemy),
            ),
        )
        .register_rollback_type::<Transform>()
        .build(&mut app);

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        });

    app.add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(NetworkingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(StructurePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(EnemyPlugin);
    //.add_startup_system(spawn_player);

    app.insert_resource(Layers::new());

    app.run();
}
