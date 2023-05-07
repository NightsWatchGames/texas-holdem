use bevy::prelude::*;
use texas_holdem_common::{Player, PlayerRole, RoomState};

use crate::{lobby::PlayerName, play::CurrentPlayInfo};

#[derive(Debug, Component)]
pub struct ParticipantRoleButton;

#[derive(Debug, Component)]
pub struct SpectatorRoleButton;

#[derive(Debug, Component)]
pub struct PlayerListUI;
#[derive(Debug, Component)]
pub struct PlayerListUIItem;

#[derive(Debug, Component)]
pub struct RoomStateUIText;
#[derive(Debug, Component)]
pub struct PlayRoundUI;
#[derive(Debug, Component)]
pub struct PlayRoundUIText;

// 当前房间信息
#[derive(Debug, Default, Resource)]
pub struct CurrentRoomInfo {
    pub room_id: u64,
    pub room_state: RoomState,
    pub my_role: PlayerRole,
    pub players: Vec<Player>,
}

impl CurrentRoomInfo {
    pub fn contains_player(&self, player_name: &str) -> bool {
        self.players
            .iter()
            .any(|player| player.player_name == player_name)
    }
}

#[derive(Debug)]
pub struct SwitchPlayerRoleEvent {
    pub room_id: u64,
    pub target_player_role: PlayerRole,
}

const NORMAL_PLAYER_ROLE_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const SELECTED_PLAYER_ROLE_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn setup_room_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_name: Res<PlayerName>,
    current_room_info: Res<CurrentRoomInfo>,
) {
    commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            // 左侧布局
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::BEIGE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 玩家角色切换
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(40.0)),
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::RED.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            let button_text_style = TextStyle {
                                font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                                font_size: 20.0,
                                ..default()
                            };
                            let button_style = Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                ..default()
                            };
                            parent
                                .spawn((
                                    ParticipantRoleButton,
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: NORMAL_PLAYER_ROLE_BUTTON_COLOR.into(),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "Participant",
                                            button_text_style.clone(),
                                        ),
                                        ..default()
                                    });
                                });
                            parent
                                .spawn((
                                    SpectatorRoleButton,
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: SELECTED_PLAYER_ROLE_BUTTON_COLOR.into(),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "Spectator",
                                            button_text_style.clone(),
                                        ),
                                        ..default()
                                    });
                                });
                        });
                    // 房间玩家信息 TODO scrollable
                    parent
                        .spawn((
                            PlayerListUI,
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Auto, Val::Px(300.0)),
                                    flex_direction: FlexDirection::ColumnReverse,
                                    margin: UiRect::all(Val::Px(10.0)),
                                    overflow: Overflow::Hidden,
                                    ..default()
                                },
                                background_color: Color::BLUE.into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                PlayerListUIItem,
                                TextBundle {
                                    text: Text::from_section(
                                        player_name.0.clone(),
                                        TextStyle {
                                            font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                },
                            ));
                        });
                });
            // 中间布局
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 房间状态
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(300.0), Val::Px(50.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::BLUE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                RoomStateUIText,
                                TextBundle {
                                    text: Text::from_section(
                                        current_room_info.room_state.name(),
                                        TextStyle {
                                            font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                                            font_size: 40.0,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                },
                            ));
                        });
                    // 对局round
                    parent
                        .spawn((
                            PlayRoundUI,
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(200.0), Val::Px(30.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::BLUE.into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                PlayRoundUIText,
                                TextBundle {
                                    text: Text::from_section(
                                        "Round - ?",
                                        TextStyle {
                                            font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                },
                            ));
                        });
                });
            // 右侧布局
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::GREEN.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 对局日志
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(200.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::AZURE.into(),
                        ..default()
                    });
                    // 玩家操作按键
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(300.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::FUCHSIA.into(),
                        ..default()
                    });
                });
        });
}

pub fn player_role_ui_system(
    mut q_participant_role_button: Query<
        (Ref<Interaction>, &mut BackgroundColor),
        With<ParticipantRoleButton>,
    >,
    mut q_spectator_role_button: Query<
        (Ref<Interaction>, &mut BackgroundColor),
        (With<SpectatorRoleButton>, Without<ParticipantRoleButton>),
    >,
    current_room_info: Res<CurrentRoomInfo>,
    mut switch_player_role_ew: EventWriter<SwitchPlayerRoleEvent>,
) {
    let (participant_role_button_interaction, mut participant_role_button_bg_color) =
        q_participant_role_button.single_mut();
    let (spectator_role_button_interaction, mut spectator_role_button_bg_color) =
        q_spectator_role_button.single_mut();

    // 点击切换角色
    if Interaction::Clicked == *participant_role_button_interaction
        && participant_role_button_interaction.is_changed()
    {
        println!("participant_role_button clicked");
        switch_player_role_ew.send(SwitchPlayerRoleEvent {
            room_id: current_room_info.room_id,
            target_player_role: PlayerRole::Participant,
        });
    } else if Interaction::Clicked == *spectator_role_button_interaction
        && spectator_role_button_interaction.is_changed()
    {
        println!("spectator_role_button clicked");
        switch_player_role_ew.send(SwitchPlayerRoleEvent {
            room_id: current_room_info.room_id,
            target_player_role: PlayerRole::Spectator,
        });
    }

    // 根据当前角色设置按钮颜色
    match current_room_info.my_role {
        PlayerRole::Participant => {
            *participant_role_button_bg_color = SELECTED_PLAYER_ROLE_BUTTON_COLOR.into();
            *spectator_role_button_bg_color = NORMAL_PLAYER_ROLE_BUTTON_COLOR.into();
        }
        PlayerRole::Spectator => {
            *participant_role_button_bg_color = NORMAL_PLAYER_ROLE_BUTTON_COLOR.into();
            *spectator_role_button_bg_color = SELECTED_PLAYER_ROLE_BUTTON_COLOR.into();
        }
    }
}

pub fn player_list_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_room_info: Res<CurrentRoomInfo>,
    mut q_player_list_ui: Query<Entity, With<PlayerListUI>>,
    q_player_list_ui_item: Query<(Entity, &Text), With<PlayerListUIItem>>,
) {
    let parent = q_player_list_ui.single_mut();
    // 删除已经不在房间的玩家
    for (entity, text) in &q_player_list_ui_item {
        if !current_room_info.contains_player(&text.sections[0].value) {
            commands.entity(entity).despawn_recursive();
        }
    }

    // 新增玩家
    for player in current_room_info.players.iter() {
        if !q_player_list_ui_item
            .iter()
            .any(|(_, text)| text.sections[0].value == player.player_name)
        {
            commands.entity(parent).with_children(|parent| {
                parent.spawn((
                    PlayerListUIItem,
                    TextBundle {
                        text: Text::from_sections(vec![
                            TextSection {
                                value: player.player_name.clone(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                            },
                            TextSection {
                                value: format!("  {}", player.player_role.name()),
                                style: TextStyle {
                                    font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                                    font_size: 10.0,
                                    ..default()
                                },
                            },
                        ]),
                        ..default()
                    },
                ));
            });
        }
    }
}

pub fn room_state_ui_system(
    mut q_room_state_text: Query<&mut Text, With<RoomStateUIText>>,
    current_room_info: Res<CurrentRoomInfo>,
) {
    for mut text in &mut q_room_state_text {
        text.sections[0].value = current_room_info.room_state.name().to_string();
    }
}

pub fn play_round_ui_system(
    mut q_play_round_ui: Query<&mut Visibility, With<PlayRoundUI>>,
    mut q_play_round_ui_text: Query<&mut Text, With<PlayRoundUIText>>,
    current_play_info: Res<CurrentPlayInfo>,
) {
    for mut visibility in &mut q_play_round_ui {
        *visibility = if current_play_info.play_id.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    for mut text in &mut q_play_round_ui_text {
        text.sections[0].value = format!("Round - {}", current_play_info.round.name());
    }
}
