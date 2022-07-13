use self::place_ui::SelectUIPlugin;
use bevy::prelude::*;

pub mod place_ui;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SelectUIPlugin);
    }
}
