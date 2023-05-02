use crate::*;
use bevy_fps_counter::FpsCounterPlugin;
use std::sync::{Mutex, MutexGuard};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FpsCounterPlugin)
            .add_startup_system(init_show_ui)
            .add_system(update_turn_ui)
            .add_system(show_captured_pieces)
            .add_system(update_material_advantage_ui);
    }
}

#[derive(Component)]
struct NextMoveText;

#[derive(Component)]
struct MaterialAdvantageText;

fn init_show_ui(mut commands: Commands, asset_server: ResMut<AssetServer>, turn: Res<Turn>) {
    let font: Handle<Font> = asset_server.load("fonts/UbuntuMonoNerdFontCompleteMono.ttf");

    // Turn
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                format!(
                    "Turn:  {:?}\n       {}",
                    turn.get_color(),
                    turn.get_number_as_ordinal()
                ),
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: match turn.get_color() {
                        PieceColor::White => Color::WHITE,
                        PieceColor::Black => Color::BLACK,
                    },
                },
            ),
            ..default()
        })
        .insert(NextMoveText);

    // Material advantage
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                "Material advantage for white: 0",
                TextStyle {
                    font,
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        })
        .insert(MaterialAdvantageText);
}

fn update_turn_ui(turn: Res<Turn>, mut query: Query<&mut Text, With<NextMoveText>>) {
    if !turn.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Turn:  {:?}\n       {}",
            turn.get_color(),
            turn.get_number_as_ordinal()
        );
        text.sections[0].style.color = match turn.get_color() {
            PieceColor::White => Color::WHITE,
            PieceColor::Black => Color::BLACK,
        };
    }
}

/// This component is used to mark pieces that are captured on the side of the board
#[derive(Component)]
struct CapturedSideBoard;

static mut WHITE_CAPTURED_PIECES: Mutex<Vec<PieceType>> = Mutex::new(Vec::new());
static mut BLACK_CAPTURED_PIECES: Mutex<Vec<PieceType>> = Mutex::new(Vec::new());

/// This system shows the captured pieces on the side of the board
fn show_captured_pieces(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    captured_pieces_query: Query<&Piece, With<Captured>>,
    captured_pieces_side_board_query: Query<Entity, With<CapturedSideBoard>>,
    turn: Res<Turn>,
) {
    if turn.is_changed() {
        for piece in captured_pieces_side_board_query.iter() {
            commands.entity(piece).despawn_recursive();
        }
    } else {
        return;
    }

    for piece in captured_pieces_query.iter() {
        match piece.color {
            PieceColor::White => unsafe {
                WHITE_CAPTURED_PIECES.lock().unwrap().push(piece.piece_type)
            },
            PieceColor::Black => unsafe {
                BLACK_CAPTURED_PIECES.lock().unwrap().push(piece.piece_type)
            },
        };
    }

    unsafe {
        WHITE_CAPTURED_PIECES.lock().unwrap().sort();
        BLACK_CAPTURED_PIECES.lock().unwrap().sort();
    }

    macro_rules! load_piece {
        ($piece:ident) => {
            let $piece: Handle<Image> =
                asset_server.load(format!("chess-2d-pieces/{}.png", stringify!($piece)));
        };
    }

    load_piece!(pawn_black);
    load_piece!(pawn_white);
    load_piece!(rook_black);
    load_piece!(rook_white);
    load_piece!(knight_black);
    load_piece!(knight_white);
    load_piece!(bishop_black);
    load_piece!(bishop_white);
    load_piece!(queen_black);
    load_piece!(queen_white);

    let piece_scale: Vec3 = Vec3::new(0.02, 0.02, 1.);
    let square_size: f32 = 60.;

    // Rotate captured pieces when it's black's turn
    // This is needed to be done here because this function is called every frame
    let rotation: Quat = match turn.get_color() {
        PieceColor::White => Quat::default(),
        PieceColor::Black => Quat::from_rotation_z(std::f32::consts::PI),
    };

    for (i, piece) in unsafe { &WHITE_CAPTURED_PIECES }
        .lock()
        .unwrap()
        .iter()
        .enumerate()
    {
        let piece_pos: Vec3 = Vec3::new(-3.8 * square_size + i as f32 * 16., 4.2 * square_size, 0.);
        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: piece_pos,
                    scale: piece_scale,
                    rotation,
                },
                texture: match piece {
                    PieceType::PawnWhite => pawn_white.clone(),
                    PieceType::RookWhite => rook_white.clone(),
                    PieceType::KnightWhite => knight_white.clone(),
                    PieceType::BishopWhite => bishop_white.clone(),
                    PieceType::QueenWhite => queen_white.clone(),
                    _ => continue,
                },
                ..default()
            })
            .insert(CapturedSideBoard);
    }
    for (i, piece) in unsafe { &BLACK_CAPTURED_PIECES }
        .lock()
        .unwrap()
        .iter()
        .enumerate()
    {
        let piece_pos: Vec3 = Vec3::new(3.8 * square_size - i as f32 * 16., -4.2 * square_size, 0.);
        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: piece_pos,
                    scale: piece_scale,
                    rotation,
                },
                texture: match piece {
                    PieceType::PawnBlack => pawn_black.clone(),
                    PieceType::RookBlack => rook_black.clone(),
                    PieceType::KnightBlack => knight_black.clone(),
                    PieceType::BishopBlack => bishop_black.clone(),
                    PieceType::QueenBlack => queen_black.clone(),
                    _ => continue,
                },
                ..default()
            })
            .insert(CapturedSideBoard);
    }
}

fn update_material_advantage_ui(
    turn: Res<Turn>,
    mut query: Query<&mut Text, With<MaterialAdvantageText>>,
) {
    if !turn.is_changed() {
        return;
    }

    let white_captured_pieces: MutexGuard<Vec<PieceType>> =
        unsafe { &WHITE_CAPTURED_PIECES }.lock().unwrap();
    let black_captured_pieces: MutexGuard<Vec<PieceType>> =
        unsafe { &BLACK_CAPTURED_PIECES }.lock().unwrap();

    // Calculate material advantage
    let material_advantage: i8 = black_captured_pieces
        .iter()
        .map(|piece| piece.get_value())
        .sum::<i8>()
        - white_captured_pieces
            .iter()
            .map(|piece| piece.get_value())
            .sum::<i8>();

    // Show material advantage
    let mut text = query.single_mut();
    text.sections[0].value = format!("Material advantage for white: {}", material_advantage);
}
