use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use egui_extras::{Column, TableBuilder};

use crate::room::RoomList;

#[derive(Debug, Component)]
pub struct LobbyRoomListUI;

pub fn lobby_room_list_ui(mut contexts: EguiContexts, room_list: Res<RoomList>) {
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
                    });
                }
            });
    });
}
