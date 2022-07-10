use bevy::{app::*, ecs::prelude::*};
use kayak_ui::{
    bevy::*,
    core::{styles::*, *},
    widgets::*,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(render_ui);
    }
}

fn render_ui(mut cmd: Commands) {
    let context = BevyContext::new(|context| {
        let style = Style {
            padding_left: StyleProp::Value(Units::Pixels(100.)),
            ..Style::default()
        };
        render! {
            <kayak_ui::widgets::App>
                <Window styles={Some(style)}>
                    <Text size={30.0} content={"hello".into()} />
                </Window>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}
