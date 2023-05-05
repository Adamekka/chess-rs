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
            .add_system(update_material_advantage_ui)
            .add_system(update_enable_ai_button_ui);
    }
}

#[derive(Component)]
struct NextMoveText;

#[derive(Component)]
struct MaterialAdvantageText;

#[derive(Component)]
pub struct AIEnabled(pub bool);

const AI_BUTTON_ENABLED: BackgroundColor = BackgroundColor(Color::rgb(0.35, 0.75, 0.35));
const AI_BUTTON_ENABLED_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.45, 0.85, 0.45));
const AI_BUTTON_DISABLED: BackgroundColor = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
const AI_BUTTON_DISABLED_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));

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
                format!("Turn:  {:?}\n       {}", turn.color, Ordinal(turn.n)),
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: match turn.color {
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
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        })
        .insert(MaterialAdvantageText);

    // AI button
    commands
        .spawn(ButtonBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(50.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            background_color: AI_BUTTON_DISABLED,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "AI disabled",
                TextStyle {
                    font,
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        })
        .insert(AIEnabled(false));
}

fn update_turn_ui(turn: Res<Turn>, mut query: Query<&mut Text, With<NextMoveText>>) {
    if !turn.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Turn:  {:?}\n       {}", turn.color, Ordinal(turn.n));
        text.sections[0].style.color = match turn.color {
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
    let rotation: Quat = match turn.color {
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

#[allow(clippy::type_complexity)]
fn update_enable_ai_button_ui(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &mut AIEnabled,
        ),
        (Changed<Interaction>, With<AIEnabled>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children, mut ai_enabled) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Clicked => match ai_enabled.0 {
                true => {
                    color.0 = AI_BUTTON_DISABLED.0;
                    text.sections[0].value = "AI disabled".to_string();
                    ai_enabled.0 = false;
                }
                false => {
                    color.0 = AI_BUTTON_ENABLED.0;
                    text.sections[0].value = "AI enabled".to_string();
                    ai_enabled.0 = true;
                }
            },
            Interaction::Hovered => match ai_enabled.0 {
                true => {
                    color.0 = AI_BUTTON_ENABLED_HOVER.0;
                }
                false => {
                    color.0 = AI_BUTTON_DISABLED_HOVER.0;
                }
            },
            Interaction::None => match ai_enabled.0 {
                true => {
                    color.0 = AI_BUTTON_ENABLED.0;
                }
                false => {
                    color.0 = AI_BUTTON_DISABLED.0;
                }
            },
        }
    }
}
