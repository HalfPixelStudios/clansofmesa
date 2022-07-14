use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar {
    percent: f32,
    dimension: Vec2, // dimension: Vec2,
                     // bg_color: Color,
                     // fg_color: Color,
}

impl HealthBar {
    pub fn new(dimension: Vec2) -> Self {
        HealthBar {
            percent: 1.,
            dimension,
        }
    }
}

#[derive(Component)]
struct HealthBarForeground;

#[derive(Component)]
struct HealthBarBackground;

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(health_bar_system);
    }
}

pub fn spawn_health_bar(
    mut cmd: Commands,
    dimension: Vec2,
    bg_color: Color,
    fg_color: Color,
) -> Entity {
    let parent = cmd.spawn().id();
    cmd.entity(parent).insert(HealthBar::new(dimension));

    let fg = cmd.spawn().id();
    cmd.entity(fg)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: bg_color,
                custom_size: Some(dimension),
                ..default()
            },
            ..default()
        })
        .insert(HealthBarForeground);

    let bg = cmd.spawn().id();
    cmd.entity(bg)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: fg_color,
                custom_size: Some(dimension),
                ..default()
            },
            ..default()
        })
        .insert(HealthBarBackground);

    cmd.entity(parent).push_children(&[fg, bg]);

    return parent;
}

fn health_bar_system(
    parent_query: Query<(&HealthBar, &Children)>,
    mut fg_query: Query<&mut Sprite, With<HealthBarForeground>>,
) {
    for (health_bar, children) in parent_query.iter() {
        for &child in children.iter() {
            // update the width of the foreground
            if let Ok(mut sprite) = fg_query.get_mut(child) {
                sprite.custom_size = Some(Vec2::new(
                    health_bar.dimension.x * health_bar.percent,
                    health_bar.dimension.y,
                ));
            }
        }
    }
}
