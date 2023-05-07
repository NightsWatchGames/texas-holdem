use bevy::prelude::*;
use texas_holdem_common::PlayerRole;

#[derive(Debug, Component)]
pub struct PlayerRoleUI;

#[derive(Debug, Component)]
pub struct ParticipantRoleButton;

#[derive(Debug, Component)]
pub struct SpectatorRoleButton;

// 当前房间信息
#[derive(Debug, Default, Resource)]
pub struct CurrentRoomInfo {
    pub room_id: u64,
    pub my_role: PlayerRole,
}

#[derive(Debug)]
pub struct SwitchPlayerRoleEvent {
    pub room_id: u64,
    pub target_player_role: PlayerRole,
}

const NORMAL_PLAYER_ROLE_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const SELECTED_PLAYER_ROLE_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn setup_room_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    // 房间玩家信息
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(300.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::BLUE.into(),
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Player1",
                                TextStyle {
                                    font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
                });
            // 中间布局
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 房间状态
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(300.0), Val::Px(50.0)),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::BLUE.into(),
                        ..default()
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