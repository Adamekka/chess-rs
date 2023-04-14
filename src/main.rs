use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::{
    DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent, SelectionEvent,
};

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
        .init_resource::<SelectedSquare>()
        .init_resource::<SelectedPiece>()
        .init_resource::<Turn>()
        .add_plugins(DefaultPickingPlugins)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_startup_system(setup)
        .add_system(select_square)
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

#[derive(Clone, Copy, PartialEq)]
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
    None,
}

#[derive(Clone, Copy, PartialEq)]
enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Component, Debug, Default)]
struct Square {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy, Component)]
struct Piece {
    piece_type: PieceType,
    color: PieceColor,
    square: Square,
}

#[derive(Resource)]
struct Turn {
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

    fn next_turn(&mut self) {
        self.color = match self.color {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        self.n += 1;
    }

    fn get_turn_color(&self) -> PieceColor {
        self.color
    }

    fn get_turn_number(&self) -> u16 {
        self.n
    }
}

impl Default for Turn {
    fn default() -> Self {
        Self::new()
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands
        .spawn(Camera2dBundle::default())
        .insert(PickingCameraBundle::default());

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

    for row in 0..n_of_squares {
        for column in 0..n_of_squares {
            let piece_type = &piece_positions[row as usize][column as usize];
            let square_pos = Vec3::new(
                row as f32 * square_size - board_half_width + square_size / 2.0,
                column as f32 * square_size - board_half_width + square_size / 2.0,
                0.,
            );
            let (material, color) = if (row + column) % 2 == 0 {
                (&black_material, PieceColor::Black)
            } else {
                (&white_material, PieceColor::White)
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
                .insert(Square { y: row, x: column });

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
                    color,
                    square: { Square { y: row, x: column } },
                });
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
