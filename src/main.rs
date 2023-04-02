use bevy::prelude::*;

const WINDOW_TITLE: &str = "Chess by Adamekka";
const WINDOW_WIDTH: u16 = 1280;
const WINDOW_HEIGHT: u16 = 720;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..Default::default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
