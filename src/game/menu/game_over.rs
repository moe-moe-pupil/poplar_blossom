
use bevy:: prelude::*;

use crate::AppState;
use crate::game::menu::ButtonColors;
use crate::game::menu::ChangeState;

pub fn game_over_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::GameOverMenu), on_game_over)
        .add_systems(Update, (click_button).run_if(in_state(AppState::GameOverMenu)))
        .add_systems(OnExit(AppState::GameOverMenu), cleanup_menu);
}



#[derive(Component)]
struct GameOverMenu;

#[derive(Component)]
enum ButtonType {
    Play,
    ToMenu,
}

fn on_game_over(
    mut commands: Commands,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(5.),
                    ..default()
                },
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(AppState::Playing),
                    ButtonType::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "One More Game",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
                
            let button_colors2 = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors2.normal.into(),
                        ..Default::default()
                    },
                    button_colors2,
                    ButtonType::ToMenu,
                    ChangeState(AppState::MainMenu),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Back to Menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

fn click_button(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (
            &ButtonType,
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (_button_type, interaction, mut color, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands, 
    menu: Query<Entity, With<GameOverMenu>>,
) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
     
}