use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Player {
    X,
    O,
}

#[derive(Resource)]
struct CurrentTurn(Player);

#[derive(Component)]
struct CellState(Option<Player>);

#[derive(Component)]
enum MiniBoardState {
    Unclaimed,
    Claimed(Player),
    Drawn,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [900.0, 900.0].into(),
                title: "MegaTicTacToe".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(CurrentTurn(Player::X))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_click,
                update_board,
                check_miniboards,
                update_miniboards,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::flex(1.0); 3],
                grid_template_rows: vec![GridTrack::flex(1.0); 3],
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        })
        .with_children(|parent| {
            for _ in 0..9 {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                border: UiRect::all(Val::Px(7.0)),
                                grid_template_columns: vec![GridTrack::flex(1.0); 3],
                                grid_template_rows: vec![GridTrack::flex(1.0); 3],
                                ..Default::default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            ..Default::default()
                        },
                        MiniBoardState::Unclaimed,
                    ))
                    .with_children(|parent| {
                        for _ in 0..9 {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            display: Display::Grid,
                                            border: UiRect::all(Val::Px(3.0)),
                                            align_items: AlignItems::Center,
                                            justify_items: JustifyItems::Center,
                                            ..Default::default()
                                        },
                                        border_color: BorderColor(Color::DARK_GRAY),
                                        ..Default::default()
                                    },
                                    CellState(None),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "",
                                        TextStyle {
                                            font_size: 80.0,
                                            color: Color::BLACK,
                                            ..Default::default()
                                        },
                                    ));
                                });
                        }
                    });
            }
        });
}

fn handle_click(
    mut query: Query<(&Interaction, &mut CellState), Changed<Interaction>>,
    mut current_turn: ResMut<CurrentTurn>,
) {
    for (&interaction, mut cell_state) in &mut query {
        if interaction != Interaction::Pressed || cell_state.0.is_some() {
            continue;
        }

        cell_state.0 = Some(current_turn.0);
        current_turn.0 = match current_turn.0 {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}

fn update_board(
    mut cell_query: Query<(&Children, &CellState, &mut BackgroundColor), Changed<CellState>>,
    mut text_query: Query<&mut Text>,
) {
    for (children, CellState(state), mut background_color) in &mut cell_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match state {
            Some(Player::X) => {
                text.sections[0].value = "X".to_string();
                background_color.0 = Color::hex("#ef8f8f").unwrap();
            }
            Some(Player::O) => {
                text.sections[0].value = "O".to_string();
                background_color.0 = Color::hex("#8fefef").unwrap();
            }
            None => {
                text.sections[0].value = String::new();
                background_color.0 = Color::WHITE;
            }
        }
    }
}

fn find_winner(states: &[Option<Player>]) -> Option<Player> {
    let mut winner = None;

    for i in 0..3 {
        if states[i] == states[i + 3] && states[i] == states[i + 6] {
            winner = winner.or(states[i]);
        }
    }

    for i in 0..3 {
        if states[i * 3] == states[i * 3 + 1] && states[i * 3] == states[i * 3 + 2] {
            winner = winner.or(states[i * 3]);
        }
    }

    if states[0] == states[4] && states[0] == states[8] {
        winner = winner.or(states[0]);
    }

    if states[2] == states[4] && states[2] == states[6] {
        winner = winner.or(states[2]);
    }

    winner
}

fn check_miniboards(
    updated_query: Query<&Parent, Changed<CellState>>,
    mut miniboard_query: Query<(&Children, &mut MiniBoardState)>,
    cells_query: Query<&CellState>,
) {
    for parent in &updated_query {
        let (children, mut miniboard_state) = miniboard_query.get_mut(parent.get()).unwrap();
        let cell_states = cells_query
            .iter_many(children)
            .map(|state| state.0)
            .collect::<Vec<_>>();

        let Some(winner) = find_winner(&cell_states) else {
            if cell_states.iter().flatten().count() == 9 {
                *miniboard_state = MiniBoardState::Drawn;
            }
            continue;
        };

        *miniboard_state = MiniBoardState::Claimed(winner);
    }
}

fn update_miniboards(
    mut commands: Commands,
    mut miniboard_query: Query<
        (Entity, &Children, &mut MiniBoardState, &mut BackgroundColor),
        Changed<MiniBoardState>,
    >,
    mut cells_query: Query<&mut CellState>,
) {
    for (miniboard, children, mut state, mut color) in &mut miniboard_query {
        let mut miniboard = commands.entity(miniboard);

        match *state {
            MiniBoardState::Claimed(Player::X) => {
                miniboard.despawn_descendants().with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "X",
                            TextStyle {
                                font_size: 240.0,
                                color: Color::BLACK,
                                ..Default::default()
                            },
                        )
                        .with_style(Style {
                            grid_column: GridPlacement::span(3),
                            grid_row: GridPlacement::span(3),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            ..Default::default()
                        }),
                    );
                });
                color.0 = Color::hex("#ef8f8f").unwrap();
            }
            MiniBoardState::Claimed(Player::O) => {
                miniboard.despawn_descendants().with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "O",
                            TextStyle {
                                font_size: 240.0,
                                color: Color::BLACK,
                                ..Default::default()
                            },
                        )
                        .with_style(Style {
                            grid_column: GridPlacement::span(3),
                            grid_row: GridPlacement::span(3),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            ..Default::default()
                        }),
                    );
                });
                color.0 = Color::hex("#8fefef").unwrap();
            }
            MiniBoardState::Drawn => {
                let mut cell_states = cells_query.iter_many_mut(children);
                while let Some(mut cell_state) = cell_states.fetch_next() {
                    cell_state.0 = None;
                }
                *state = MiniBoardState::Unclaimed;
            }
            _ => {}
        }
    }
}
