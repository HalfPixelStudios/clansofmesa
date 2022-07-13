use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use std::time::Duration;

const PLAYER1: usize = 0;
const PLAYER2: usize = 1;
pub struct LocalPlayerHandle {
    pub id: usize,
    pub mode: Mode,
}

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
pub fn run_if_defender(player: Res<LocalPlayerHandle>, game_data: Res<GameData>) -> ShouldRun {
    if game_data.defender == player.id {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}
pub fn run_if_attacker(player: Res<LocalPlayerHandle>, game_data: Res<GameData>) -> ShouldRun {
    if game_data.attacker == player.id {
        ShouldRun::No
    } else {
        ShouldRun::Yes
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
    }
}
