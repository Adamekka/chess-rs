mod ui;

use crate::ui::*;
use bevy::{
    app::AppExit,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_kira_audio::{AudioControl, AudioSource};
use bevy_mod_picking::{
    DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent, SelectionEvent,
};
use ordinal_type::Ordinal;

const WINDOW_TITLE: &str = "Chess by Adamekka";
const WINDOW_WIDTH: u16 = 1280;
const WINDOW_HEIGHT: u16 = 720;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..Default::default()
            }),
            ..default()
        }))
        .init_resource::<SelectedSquare>()
        .init_resource::<SelectedPiece>()
        .init_resource::<Turn>()
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_kira_audio::AudioPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_startup_system(setup)
        .add_plugin(ui::UIPlugin)
        .add_system(select_piece)
        .add_system(select_square.before(select_piece))
        .add_system(get_piece_for_move.after(select_piece))
        .add_system(move_piece.after(select_piece))
        .add_system(despawn_captured_pieces.after(move_piece))
        .add_system(promote_pieces.after(move_piece))
        .add_system(get_pieces.after(move_piece))
        .run();
}

#[derive(Debug, Default, Resource)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Debug, Default, Resource)]
struct SelectedPiece {
    entity: Option<Entity>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum PieceType {
    PawnBlack,
    PawnWhite,
    RookBlack,
    RookWhite,
    KnightBlack,
    KnightWhite,
    BishopBlack,
    BishopWhite,
    QueenBlack,
    QueenWhite,
    KingBlack,
    KingWhite,
    #[default]
    None,
}

trait SortPieceType {
    fn sort(&mut self);
}

impl SortPieceType for Vec<PieceType> {
    fn sort(&mut self) {
        self.sort_by(|a: &PieceType, b: &PieceType| {
            let a_idx: u8 = match a {
                PieceType::PawnWhite | PieceType::PawnBlack => 0,
                PieceType::KnightWhite | PieceType::KnightBlack => 1,
                PieceType::BishopWhite | PieceType::BishopBlack => 2,
                PieceType::RookWhite | PieceType::RookBlack => 3,
                PieceType::QueenWhite | PieceType::QueenBlack => 4,
                _ => 5,
            };
            let b_idx: u8 = match b {
                PieceType::PawnWhite | PieceType::PawnBlack => 0,
                PieceType::KnightWhite | PieceType::KnightBlack => 1,
                PieceType::BishopWhite | PieceType::BishopBlack => 2,
                PieceType::RookWhite | PieceType::RookBlack => 3,
                PieceType::QueenWhite | PieceType::QueenBlack => 4,
                _ => 5,
            };
            a_idx.cmp(&b_idx)
        })
    }
}

// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
trait ToFEN {
    fn to_fen(&self, turn: Res<Turn>) -> String;
}

impl ToFEN for [[PieceType; 8]; 8] {
    fn to_fen(&self, turn: Res<Turn>) -> String {
        let mut fen: String = String::new();

        for row in self.iter() {
            let mut empty_squares: u8 = 0;
            for piece in row.iter() {
                match piece {
                    PieceType::PawnWhite => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('P');
                    }
                    PieceType::PawnBlack => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('p');
                    }
                    PieceType::RookWhite => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('R');
                    }
                    PieceType::RookBlack => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('r');
                    }
                    PieceType::KnightWhite => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('N');
                    }
                    PieceType::KnightBlack => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('n');
                    }
                    PieceType::BishopWhite => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('B');
                    }
                    PieceType::BishopBlack => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('b');
                    }
                    PieceType::QueenWhite => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('Q');
                    }
                    PieceType::QueenBlack => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('q');
                    }
                    PieceType::KingWhite => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('K');
                    }
                    PieceType::KingBlack => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push('k');
                    }
                    PieceType::None => {
                        empty_squares += 1;
                    }
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }

            fen.push('/');
        }

        // Remove last '/'
        fen.pop();

        let turn = match turn.color {
            PieceColor::White => "w",
            PieceColor::Black => "b",
        };
        // Casting and en passant are not implemented yet
        // Don't calculate halfmove clock and fullmove number
        fen.push_str(&format!(" {turn} ---- - 0 0"));

        fen
    }
}

impl PieceType {
    fn get_value(&self) -> i8 {
        match self {
            PieceType::PawnWhite | PieceType::PawnBlack => 1,
            PieceType::KnightWhite | PieceType::KnightBlack => 3,
            PieceType::BishopWhite | PieceType::BishopBlack => 3,
            PieceType::RookWhite | PieceType::RookBlack => 5,
            PieceType::QueenWhite | PieceType::QueenBlack => 9,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Component, Debug, Default, PartialEq)]
struct Square {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy, Component, Debug)]
struct Piece {
    piece_type: PieceType,
    color: PieceColor,
    square: Square,
    direction: Square,
}

impl Piece {
    // Returns the possible moves for a piece
    fn is_move_valid(&self, new_position: Square, pieces: &Vec<Piece>) -> bool {
        // Checks if new position is same as current position
        if new_position == self.square {
            return false;
        }

        dbg!(&self);
        dbg!(&new_position);
        // If there's a piece of the same color in the new position, return false
        if color_of_piece(new_position, pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::KingWhite | PieceType::KingBlack => {
                // King can move one square in any direction
                // Horizontal
                ((self.square.x as i8 - new_position.x as i8).abs() == 1
                    && (self.square.y == new_position.y))
                // Vertical
                || ((self.square.y as i8 - new_position.y as i8).abs() == 1
                && (self.square.x == new_position.x))
                // Diagonal
                || ((self.square.x as i8 - new_position.x as i8).abs() == 1
                    && (self.square.y as i8 - new_position.y as i8).abs() == 1)
            }

            PieceType::QueenWhite | PieceType::QueenBlack => {
                // Queen can move any number of squares in any direction, horizontally, vertically or diagonally
                is_path_empty(self.square, new_position, pieces)
                    && ((self.square.x as i8 - new_position.x as i8).abs()
                        == (self.square.y as i8 - new_position.y as i8).abs()
                        || ((self.square.x == new_position.x && self.square.y != new_position.y)
                            || (self.square.x != new_position.x
                                && self.square.y == new_position.y)))
            }

            PieceType::BishopWhite | PieceType::BishopBlack => {
                // Bishop can move any number of squares diagonally
                is_path_empty(self.square, new_position, pieces)
                    && (self.square.x as i8 - new_position.x as i8).abs()
                        == (self.square.y as i8 - new_position.y as i8).abs()
            }

            PieceType::KnightWhite | PieceType::KnightBlack => {
                // Knight moves in an L shape
                ((self.square.x as i8 - new_position.x as i8).abs() == 2
                    && (self.square.y as i8 - new_position.y as i8).abs() == 1)
                    || ((self.square.x as i8 - new_position.x as i8).abs() == 1
                        && (self.square.y as i8 - new_position.y as i8).abs() == 2)
            }

            PieceType::RookWhite | PieceType::RookBlack => {
                // Rook can move any number of squares horizontally or vertically
                is_path_empty(self.square, new_position, pieces)
                    && ((self.square.x == new_position.x && self.square.y != new_position.y)
                        || (self.square.x != new_position.x && self.square.y == new_position.y))
            }

            PieceType::PawnWhite => {
                // 1 Square forward
                if new_position.y as i8 - self.square.y as i8 == 1
                    && (self.square.x == new_position.x)
                    && color_of_piece(new_position, pieces).is_none()
                {
                    return true;
                }

                // 2 Squares forward
                if self.square.y == 1
                    && new_position.y as i8 - self.square.y as i8 == 2
                    && (self.square.x == new_position.x)
                    && is_path_empty(self.square, new_position, pieces)
                    && color_of_piece(new_position, pieces).is_none()
                {
                    return true;
                }

                // Take piece diagonally
                if new_position.y as i8 - self.square.y as i8 == 1
                    && (self.square.x as i8 - new_position.x as i8).abs() == 1
                    && color_of_piece(new_position, pieces) == Some(PieceColor::Black)
                {
                    return true;
                }

                false
            }

            PieceType::PawnBlack => {
                // 1 Square forward
                if new_position.y as i8 - self.square.y as i8 == -1
                    && (self.square.x == new_position.x)
                    && color_of_piece(new_position, pieces).is_none()
                {
                    return true;
                }

                // 2 Squares forward
                if self.square.y == 6
                    && new_position.y as i8 - self.square.y as i8 == -2
                    && (self.square.x == new_position.x)
                    && is_path_empty(self.square, new_position, pieces)
                    && color_of_piece(new_position, pieces).is_none()
                {
                    return true;
                }

                // Take piece diagonally
                if new_position.y as i8 - self.square.y as i8 == -1
                    && (self.square.x as i8 - new_position.x as i8).abs() == 1
                    && color_of_piece(new_position, pieces) == Some(PieceColor::White)
                {
                    return true;
                }

                false
            }

            PieceType::None => unreachable!("PieceType::None is not a valid piece type"),
        }
    }
}

#[derive(Debug, Resource)]
pub struct Turn {
    color: PieceColor,
    n: u16,
}

impl Turn {
    fn new() -> Self {
        Self {
            color: PieceColor::White,
            n: 1,
        }
    }

    fn next(&mut self) {
        self.color = match self.color {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        self.n += 1;
    }
}

impl Default for Turn {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
struct Captured;

#[derive(Component)]
struct RotateEachTurn;

#[derive(Component)]
struct Promoted;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands
        .spawn(Camera2dBundle::default())
        .insert(PickingCameraBundle::default())
        .insert(RotateEachTurn);

    // Pieces
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
    load_piece!(king_black);
    load_piece!(king_white);

    // Array of piece positions
    // Starts from the bottom left corner
    // Array is row
    // Nested array is column
    let piece_positions: [[PieceType; 8]; 8] = [
        [
            PieceType::RookWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::RookBlack,
        ],
        [
            PieceType::KnightWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::KnightBlack,
        ],
        [
            PieceType::BishopWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::BishopBlack,
        ],
        [
            PieceType::QueenWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::QueenBlack,
        ],
        [
            PieceType::KingWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::KingBlack,
        ],
        [
            PieceType::BishopWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::BishopBlack,
        ],
        [
            PieceType::KnightWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::KnightBlack,
        ],
        [
            PieceType::RookWhite,
            PieceType::PawnWhite,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::None,
            PieceType::PawnBlack,
            PieceType::RookBlack,
        ],
    ];

    // Chessboard
    let black_material: Handle<ColorMaterial> =
        materials.add(ColorMaterial::from(Color::rgb(0.0, 0.0, 0.0)));
    let white_material: Handle<ColorMaterial> =
        materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 1.0)));

    let n_of_squares: u8 = 8;
    let square_size: f32 = 60.0;
    let piece_size: f32 = 0.06;

    let board_half_width = square_size * n_of_squares as f32 / 2.0;
    let piece_scale: Vec3 = Vec3::new(piece_size, piece_size, 1.);
    let square_mesh: Mesh2dHandle = meshes.add(Mesh::from(shape::Quad::default())).into();

    for column in 0..n_of_squares {
        for row in 0..n_of_squares {
            let piece_type = &piece_positions[column as usize][row as usize];
            let square_pos = Vec3::new(
                column as f32 * square_size - board_half_width + square_size / 2.0,
                row as f32 * square_size - board_half_width + square_size / 2.0,
                0.,
            );
            let material = if (column + row) % 2 == 0 {
                &black_material
            } else {
                &white_material
            };

            let piece_color: Option<PieceColor> = match piece_type {
                PieceType::PawnBlack
                | PieceType::RookBlack
                | PieceType::KnightBlack
                | PieceType::BishopBlack
                | PieceType::QueenBlack
                | PieceType::KingBlack => Some(PieceColor::Black),
                PieceType::PawnWhite
                | PieceType::RookWhite
                | PieceType::KnightWhite
                | PieceType::BishopWhite
                | PieceType::QueenWhite
                | PieceType::KingWhite => Some(PieceColor::White),
                PieceType::None => None,
            };

            // Spawn square
            commands
                .spawn((
                    MaterialMesh2dBundle {
                        transform: Transform {
                            translation: square_pos,
                            scale: Vec3::new(square_size, square_size, 1.),
                            ..default()
                        },
                        material: material.clone(),
                        mesh: square_mesh.clone(),
                        ..default()
                    },
                    PickableBundle::default(),
                ))
                .insert(Square { x: column, y: row });

            // Spawn piece
            commands
                .spawn(SpriteBundle {
                    transform: Transform {
                        translation: square_pos,
                        scale: piece_scale,
                        ..default()
                    },
                    texture: match piece_type {
                        PieceType::PawnBlack => pawn_black.clone(),
                        PieceType::PawnWhite => pawn_white.clone(),
                        PieceType::RookBlack => rook_black.clone(),
                        PieceType::RookWhite => rook_white.clone(),
                        PieceType::KnightBlack => knight_black.clone(),
                        PieceType::KnightWhite => knight_white.clone(),
                        PieceType::BishopBlack => bishop_black.clone(),
                        PieceType::BishopWhite => bishop_white.clone(),
                        PieceType::QueenBlack => queen_black.clone(),
                        PieceType::QueenWhite => queen_white.clone(),
                        PieceType::KingBlack => king_black.clone(),
                        PieceType::KingWhite => king_white.clone(),
                        PieceType::None => continue,
                    },
                    ..default()
                })
                .insert(Piece {
                    piece_type: *piece_type,
                    color: piece_color.expect(
                        "Piece should have a color, PieceType::None is skipped in the loop",
                    ),
                    square: { Square { x: column, y: row } },
                    direction: { Square { x: column, y: row } },
                })
                .insert(RotateEachTurn);
        }
    }
}

fn select_square(
    mut picking_events: EventReader<PickingEvent>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    // mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
) {
    // Check if mouse is clicked
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    for picking_event in picking_events.iter() {
        match picking_event {
            PickingEvent::Selection(selection_event) => {
                info!("Selection event");
                if let SelectionEvent::JustSelected(selected_entity) = selection_event {
                    if let Ok(_current_square) = squares_query.get(*selected_entity) {
                        info!("Square selected: {:?}", selected_entity.index());
                        selected_square.entity = Some(*selected_entity);
                    } else {
                        unreachable!("Deselected square event shouldn't be called");
                        // info!("Deselected");
                        // selected_square.entity = None;
                        // selected_piece.entity = None;
                        // break;
                    }
                }
            }
            PickingEvent::Hover(_) => {}
            PickingEvent::Clicked(_) => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn get_piece_for_move(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<bevy_kira_audio::Audio>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut turn: ResMut<Turn>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mut rotation_query: Query<&mut Transform, With<RotateEachTurn>>,
) {
    if !selected_square.is_changed() {
        return;
    }

    let square_entity: Entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square: &Square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec: Vec<Piece> = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
        let pieces_entity_vec: Vec<(Entity, Piece)> = pieces_query
            .iter_mut()
            .map(|(entity, piece)| (entity, *piece))
            .collect::<Vec<(Entity, Piece)>>();

        // Move the selected piece to selected square
        let (entity, mut piece): (Entity, Mut<Piece>) =
            if let Ok((entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
                (entity, piece)
            } else {
                return;
            };

        info!("Piece selected: {:?}", piece.piece_type);
        info!("Square selected: {:?}", square);

        if !piece.is_move_valid(*square, &pieces_vec) {
            warn!("Move not valid");
            return;
        }

        // Check if pawn is on the last row and insert it Promoted component
        // Black
        if piece.piece_type == PieceType::PawnBlack && square.y == 0 {
            piece.piece_type = PieceType::QueenBlack;
            commands.entity(entity).insert(Promoted);
        // White
        } else if piece.piece_type == PieceType::PawnWhite && square.y == 7 {
            piece.piece_type = PieceType::QueenWhite;
            commands.entity(entity).insert(Promoted);
        }

        // Check if piece of the opposite color exists in this square and remove it
        info!("Move valid");
        let mut captured: bool = false;
        for (other_entity, other_piece) in pieces_entity_vec {
            if other_piece.square == *square && other_piece.color != piece.color {
                // Mark piece as captured
                commands.entity(other_entity).insert(Captured);
                dbg!(other_entity);
                dbg!(other_piece.square);

                // Play capture sound
                let sound: Handle<AudioSource> = asset_server.load("sounds/capture.mp3");
                audio.play(sound);
                captured = true;
            }
        }

        if !captured {
            // Play move sound
            let sound: Handle<AudioSource> = asset_server.load("sounds/move.mp3");
            audio.play(sound);
        }

        dbg!(&piece);
        dbg!(&turn);

        // Continue only if it's the turn of the piece's color
        if piece.color != turn.color {
            // Deselect piece
            warn!("It's not {:?}'s turn", piece.color);
            selected_square.entity = None;
            selected_piece.entity = None;
            return;
        }

        // Set direction for piece to move to
        piece.direction = *square;
        dbg!(piece);
        dbg!(square);

        // Change turn
        turn.next();

        // Rotate camera and all pieces after each turn
        for mut rotation in rotation_query.iter_mut() {
            rotation.rotate(Quat::from_rotation_z(std::f32::consts::PI));
        }

        info!(
            "It's {:?}'s turn and it's {} turn",
            turn.color,
            Ordinal(turn.n)
        );

        // Deselect piece
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn move_piece(mut query: Query<(&mut Transform, &mut Piece)>) {
    for (mut transform, mut piece) in query.iter_mut() {
        // Get direction to move to
        // direction = where to move - where we are
        let where_to_move: Vec3 = Vec3::new(piece.direction.x as f32, piece.direction.y as f32, 0.);
        let where_is_piece: Vec3 = Vec3::new(piece.square.x as f32, piece.square.y as f32, 0.);
        let direction: Vec3 = where_to_move - where_is_piece;

        let square_size: Vec3 = Vec3::new(60., 60., 0.);

        // Only move if piece isn't already here
        if direction.length() > 0.1 {
            dbg!(&piece.square);
            dbg!(where_to_move);
            //  Move towards square
            transform.translation += direction * square_size;
            piece.square.x = where_to_move.x as u8;
            piece.square.y = where_to_move.y as u8;
        }
    }
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<Turn>,
    squares_query: Query<&Square>,
    pieces_query: Query<(Entity, &Piece)>,
) {
    if !selected_square.is_changed() {
        return;
    }

    info!("Select piece");

    let square_entity: Entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square: &Square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    // Select the piece in the currently selected square
    for (piece_entity, piece) in pieces_query.iter() {
        if piece.square == *square && piece.color == turn.color {
            // piece_entity is the entity of the piece in the currently selected square
            info!("Piece selected: {:?}", piece_entity.index());
            selected_piece.entity = Some(piece_entity);
            break;
        }
    }
}

/// Returns the color of the piece at the given position<br />
/// Returns None if there is no piece at the given position
fn color_of_piece(pos: Square, pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.square == pos {
            return Some(piece.color);
        }
    }

    None
}

fn is_path_empty(start: Square, end: Square, pieces: &Vec<Piece>) -> bool {
    // Same column
    if start.x == end.x {
        for piece in pieces {
            if piece.square.x == start.x
                && ((piece.square.y > start.y && piece.square.y < end.y)
                    || (piece.square.y > end.y && piece.square.y < start.y))
            {
                return false;
            }
        }
    }

    // Same row
    if start.y == end.y {
        for piece in pieces {
            if piece.square.y == start.y
                && ((piece.square.x > start.x && piece.square.x < end.x)
                    || (piece.square.x > end.x && piece.square.x < start.x))
            {
                return false;
            }
        }
    }

    // Diagonals
    let x_diff = (start.x as i8 - end.x as i8).abs();
    let y_diff = (start.y as i8 - end.y as i8).abs();

    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos: Square = if start.x < end.x && start.y < end.y {
                // Piece => Top right
                Square {
                    x: start.x + i as u8,
                    y: start.y + i as u8,
                }
            } else if start.x < end.x && start.y > end.y {
                // Piece => Bottom right
                Square {
                    x: start.x + i as u8,
                    y: start.y - i as u8,
                }
            } else if start.x > end.x && start.y < end.y {
                // Piece => Top left
                Square {
                    x: start.x - i as u8,
                    y: start.y + i as u8,
                }
            } else {
                // Piece => Bottom left
                Square {
                    x: start.x - i as u8,
                    y: start.y - i as u8,
                }
            };

            if color_of_piece(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

fn despawn_captured_pieces(
    mut commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    query: Query<(Entity, &Piece, With<Captured>)>,
) {
    for (entity, piece, _) in query.iter() {
        info!("Despawn captured piece: {:?}", entity.index());
        if piece.piece_type == PieceType::KingWhite || piece.piece_type == PieceType::KingBlack {
            println!("Thanks for playing!");
            println!(
                "{} won!",
                match piece.color {
                    PieceColor::White => "Black",
                    PieceColor::Black => "White",
                }
            );
            app_exit_events.send(AppExit);
        }

        // Despawn captured piece
        commands.entity(entity).despawn_recursive();
    }
}

fn promote_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    turn: Res<Turn>,
    mut pieces_query: Query<(&mut Handle<Image>, Entity), With<Promoted>>,
) {
    if !turn.is_changed() {
        return;
    }

    // Load queen image
    match turn.color {
        PieceColor::White => {
            let piece = pieces_query.get_single_mut();
            if let Ok((mut piece, entity)) = piece {
                info!("Promoted piece!");
                *piece = asset_server.load("chess-2d-pieces/queen_white.png");
                commands.get_entity(entity).unwrap().remove::<Promoted>();
            }
        }
        PieceColor::Black => {
            let piece = pieces_query.get_single_mut();
            if let Ok((mut piece, entity)) = piece {
                info!("Promoted piece!");
                *piece = asset_server.load("chess-2d-pieces/queen_black.png");
                commands.get_entity(entity).unwrap().remove::<Promoted>();
            }
        }
    }
}

fn get_pieces(
    ai_enabled_query: Query<&AIEnabled, With<AIEnabled>>,
    pieces_query: Query<&Piece>,
    turn: Res<Turn>,
) {
    if !turn.is_changed() {
        return;
    }

    let ai_enabled = ai_enabled_query.get_single().unwrap();
    if !ai_enabled.0 {
        return;
    }

    let mut pieces_pos: [[PieceType; 8]; 8] = Default::default();

    for piece in pieces_query.iter() {
        pieces_pos[(-(piece.square.y as i8) + 7) as usize][piece.square.x as usize] =
            piece.piece_type;
    }

    dbg!(pieces_pos);

    println!("Fen: {}", pieces_pos.to_fen(turn));
}
