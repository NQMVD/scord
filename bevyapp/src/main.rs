use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod data;
mod ui;
mod scoring;
mod import_export;

use data::{AppState, EditingState};
use ui::ui_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "scord - Contestant Scoring".to_string(),
                resolution: (1400.0, 900.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_resource::<AppState>()
        .init_resource::<EditingState>()
        .add_systems(Startup, setup_app)
        .add_systems(Update, ui_system)
        .run();
}

fn setup_app(mut app_state: ResMut<AppState>) {
    app_state.new_property_weight = 1.0;
    app_state.new_property_higher_is_better = true;
}