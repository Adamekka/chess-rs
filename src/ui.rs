use crate::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_show_turn_ui_text)
            .add_system(update_turn_ui_text);
    }
}

#[derive(Component)]
struct NextMoveText;

fn init_show_turn_ui_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    turn: Res<Turn>,
) {
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

fn update_turn_ui_text(turn: Res<Turn>, mut query: Query<&mut Text, With<NextMoveText>>) {
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
