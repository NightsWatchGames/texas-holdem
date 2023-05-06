use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, pos2},
    EguiContexts,
};
use egui_extras::{Column, TableBuilder};
use texas_holdem_common::RoomDTO;

// 房间列表
#[derive(Debug, Resource)]
pub struct RoomList(pub Vec<RoomDTO>);

// 玩家名称
#[derive(Debug, Resource)]
pub struct PlayerName(pub String);

// 新房间设置
#[derive(Debug, Default, Resource)]
pub struct NewRoomSettings {
    pub room_name: String,
    pub room_password: String,
}

// 准备进入的房间
#[derive(Debug, Default, Resource)]
pub struct RoomToEnter {
    pub room_id: Option<u64>,
    pub room_password: String,
}

// 是否打开输入密码的弹窗
#[derive(Debug, Default, Resource)]
pub struct InputPasswordModalOpen(pub bool);

#[derive(Debug, Default)]
pub struct CreateRoomEvent;

#[derive(Debug, Default)]
pub struct EnterRoomEvent {
    pub room_id: u64,
    pub room_password: String,
}

pub fn lobby_room_list_ui(
    mut contexts: EguiContexts,
    room_list: Res<RoomList>,
    mut room_to_enter: ResMut<RoomToEnter>,
    mut input_password_modal_open: ResMut<InputPasswordModalOpen>,
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("Room List");
        ui.separator();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            // .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::initial(100.0).range(40.0..=300.0))
            .column(Column::initial(100.0).at_least(40.0).clip(true))
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Room Name");
                });
                header.col(|ui| {
                    ui.strong("Owner Name");
                });
                header.col(|ui| {
                    ui.strong("Player Count");
                });
                header.col(|ui| {
                    ui.strong("Operations");
                });
            })
            .body(|mut body| {
                for room in room_list.0.iter() {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(room.room_name.clone());
                        });
                        row.col(|ui| {
                            ui.label(room.owner_name.clone());
                        });
                        row.col(|ui| {
                            ui.label(room.player_count.to_string());
                        });
                        row.col(|ui| {
                            if ui.button("Enter").clicked() {
                                println!("Enter Room: {}", room.room_name);
                                room_to_enter.room_id = Some(room.room_id);
                                input_password_modal_open.0 = true;
                            }
                        });
                    });
                }
            });
    });
}

pub fn lobby_enter_room_modal_ui(
    mut contexts: EguiContexts,
    mut room_to_enter: ResMut<RoomToEnter>,
    mut input_password_modal_open: ResMut<InputPasswordModalOpen>,
    mut enter_room_ew: EventWriter<EnterRoomEvent>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = primary_window.single();
    egui::Window::new("Modal Window")
        .open(&mut input_password_modal_open.0)
        .current_pos(pos2(
            primary_window.width() / 2. - 120.,
            primary_window.height() / 2. - 100.,
        ))
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Input Password");
            ui.add(egui::TextEdit::singleline(&mut room_to_enter.room_password));
            if ui.button("Enter").clicked() {
                enter_room_ew.send(EnterRoomEvent {
                    room_id: room_to_enter.room_id.unwrap(),
                    room_password: room_to_enter.room_password.clone(),
                });
                room_to_enter.room_id = None;
                room_to_enter.room_password = "".to_string();
            }
        });
}

pub fn lobby_create_room_ui(
    mut contexts: EguiContexts,
    mut new_room_settings: ResMut<NewRoomSettings>,
    mut create_room_ew: EventWriter<CreateRoomEvent>,
) {
    egui::Window::new("Hello2").show(contexts.ctx_mut(), |ui| {
        ui.label("Create Room");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Room Name: ");
            ui.add(egui::TextEdit::singleline(&mut new_room_settings.room_name));
        });
        ui.horizontal(|ui| {
            ui.label("Room Password: ");
            ui.add(egui::TextEdit::singleline(
                &mut new_room_settings.room_password,
            ));
        });
        if ui.button("Create").clicked() {
            create_room_ew.send_default();
        }
    });
}

pub fn lobby_set_player_name_ui(mut contexts: EguiContexts, mut player_name: ResMut<PlayerName>) {
    egui::Window::new("Hello3").show(contexts.ctx_mut(), |ui| {
        ui.label("Set Your Name");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.add(
                egui::TextEdit::singleline(&mut player_name.0).hint_text("Write something here"),
            );
        });
    });
}
