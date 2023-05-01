use crate::*;
use bevy_fps_counter::FpsCounterPlugin;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FpsCounterPlugin)
            .add_startup_system(init_show_ui)
            .add_system(update_turn_ui)
            .add_system(show_captured_pieces);
    }
}

#[derive(Component)]
struct NextMoveText;

fn init_show_ui(mut commands: Commands, asset_server: ResMut<AssetServer>, turn: Res<Turn>) {
    let font: Handle<Font> = asset_server.load("fonts/UbuntuMonoNerdFontCompleteMono.ttf");

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
                    font,
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

static mut WHITE_CAPTURED_PIECES: Vec<PieceType> = Vec::new();
static mut BLACK_CAPTURED_PIECES: Vec<PieceType> = Vec::new();

fn show_captured_pieces(captured_pieces_query: Query<&Piece, With<Captured>>) {
    for piece in captured_pieces_query.iter() {
        match piece.color {
            PieceColor::White => unsafe { WHITE_CAPTURED_PIECES.push(piece.piece_type) },
            PieceColor::Black => unsafe { BLACK_CAPTURED_PIECES.push(piece.piece_type) },
        };
    }

    unsafe {
        WHITE_CAPTURED_PIECES.sort();
        BLACK_CAPTURED_PIECES.sort();
    }
}
