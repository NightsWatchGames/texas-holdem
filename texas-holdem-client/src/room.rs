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

pub fn setup_player_role_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            PlayerRoleUI,
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    margin: UiRect {
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            let button_text_style = TextStyle {
                font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                font_size: 20.0,
                ..default()
            };
            let button_style = Style {
                size: Size::new(Val::Auto, Val::Px(40.0)),
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
                        text: Text::from_section("Participant", button_text_style.clone()),
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
                        text: Text::from_section("Spectator", button_text_style.clone()),
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
