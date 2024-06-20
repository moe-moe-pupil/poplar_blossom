use bevy::prelude::*;

use crate::AppState;

pub mod main;
pub mod room;
pub mod game_over;

pub struct MenuPlugin;

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}


#[derive(Component)]
struct ChangeState(AppState);

impl Plugin for MenuPlugin {
    
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                main::main_menu_plugin,
                game_over::game_over_menu_plugin,
            ));
    }
}