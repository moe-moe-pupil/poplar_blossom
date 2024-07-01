use crate::AppState;
use bevy:: prelude::*;

pub fn effect_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Playing), setup_effect)
        .add_systems(Update, (disappearing).run_if(in_state(AppState::Playing)))
        // .add_systems(OnExit(AppState::Playing), cleanup_effect)
        ;
}

fn setup_effect(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(5.0),
                left: Val::Percent(25.),
                top: Val::Percent(40.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        },
        Disappearing {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Name::new("tetttttt")
    ));
}

#[derive(Component)]
pub struct Disappearing {
    pub timer: Timer,
}

fn disappearing(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackgroundColor, &mut Disappearing)>,
    time: Res<Time>,
) {
    for (e, mut background_color, mut disappearing) in query.iter_mut() {
        disappearing.timer.tick(time.delta());
        
        if disappearing.timer.finished() {
            let mut color_a = background_color.0.a();
            color_a -= 0.02;
            // println!("disappearing {}", color_a);
            background_color.0.set_a(color_a);
            if color_a <= 0.0 {
                commands.entity(e).despawn();
            }
        }
    }
}
