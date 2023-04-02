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
    // Chessboard
    let black_material = Color::rgb(0.0, 0.0, 0.0);
    let white_material = Color::rgb(1.0, 1.0, 1.0);

    let n_of_squares: u8 = 8;
    let square_size: f32 = 60.0;

    let board_half_width = square_size * n_of_squares as f32 / 2.0;

    for row in 0..n_of_squares {
        for column in 0..n_of_squares {
            let square_pos = Vec2::new(
                row as f32 * square_size - board_half_width + square_size / 2.0,
                column as f32 * square_size - board_half_width + square_size / 2.0,
            );
            let material = if (row + column) % 2 == 0 {
                &black_material
            } else {
                &white_material
            };
            commands.spawn(SpriteBundle {
                transform: Transform {
                    translation: square_pos.extend(0.0),
                    scale: Vec3::new(square_size, square_size, 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: material.clone(),
                    ..default()
                },
                ..default()
            });
        }
    }
}
