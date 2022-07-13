use super::input::*;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use ggrs::InputStatus;
use std::time::Duration;

const PLAYER1: usize = 0;
const PLAYER2: usize = 1;
pub struct LocalPlayerHandle {
    pub id: usize,
    pub mode: Mode,
}

#[derive(PartialEq)]
pub enum Mode {
    Building,
    Deploying,
    Camera,
}

pub struct GameData {
    pub round_timer: Timer,
    pub break_timer: Timer,
    pub round: i32,
    pub scores: [i32; 2],
    pub attacker: usize,
    pub defender: usize,
}
impl Default for GameData {
    fn default() -> Self {
        Self {
            round_timer: Timer::new(Duration::from_secs(69), true),
            break_timer: Timer::new(Duration::from_secs(5), true),
            round: 0,
            scores: [0; 2],
            attacker: PLAYER1,
            defender: PLAYER2,
        }
    }
}
pub fn run_if_defender(
    player: Option<Res<LocalPlayerHandle>>,
    game_data: Res<GameData>,
) -> ShouldRun {
    match player {
        Some(p) => {
            if game_data.defender == p.id {
                ShouldRun::No
            } else {
                ShouldRun::Yes
            }
        }
        None => ShouldRun::No,
    }
}
pub fn run_if_camera(player: Option<Res<LocalPlayerHandle>>) -> ShouldRun {
    match player {
        Some(p) => {
            if p.mode == Mode::Camera {
                ShouldRun::Yes
            } else {
                ShouldRun::No
            }
        }
        None => ShouldRun::No,
    }
}
pub fn run_if_attacker(
    player: Option<Res<LocalPlayerHandle>>,
    game_data: Res<GameData>,
) -> ShouldRun {
    match player {
        Some(p) => {
            if game_data.attacker == p.id {
                ShouldRun::No
            } else {
                ShouldRun::Yes
            }
        }
        None => ShouldRun::No,
    }
}
pub fn run_if_action(
    player: Option<Res<LocalPlayerHandle>>,
    game_data: Res<GameData>,
) -> ShouldRun {
    match player {
        Some(p) => {
            if check_action(p, game_data) {
                ShouldRun::Yes
            } else {
                ShouldRun::No
            }
        }
        None => ShouldRun::No,
    }
}
pub fn check_action(player: Res<LocalPlayerHandle>, game_data: Res<GameData>) -> bool {
    if (game_data.attacker == player.id && player.mode == Mode::Deploying)
        || (game_data.defender == player.id && player.mode == Mode::Building)
    {
        true
    } else {
        false
    }
}
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameData::default())
            .add_system(tick_round);
    }
}

pub fn tick_round(time: Res<Time>, mut game_data: ResMut<GameData>) {
    game_data.round_timer.tick(time.delta());
    if game_data.round_timer.just_finished() {
        game_data.round += 1;
        let b = game_data.attacker;
        game_data.attacker = game_data.defender;
        game_data.defender = b;
    }
}
pub fn change_mode(
    mut player: ResMut<LocalPlayerHandle>,
    inputs: Res<Vec<(NetInput, InputStatus)>>,
    game_data: ResMut<GameData>,
) {
    let (input, _) = inputs[player.id];
    if (input.pressed & ACTION != 0) {
        if game_data.attacker == player.id {
            player.mode = Mode::Deploying;
        } else {
            player.mode = Mode::Building;
        }
    } else if (input.pressed & CAMERA != 0) {
        player.mode = Mode::Camera;
    }
}
