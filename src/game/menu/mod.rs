use bevy::prelude::*;

pub mod main;
pub mod room;
pub mod game_finish;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                main::main_menu_plugin,
            ));
    }
}