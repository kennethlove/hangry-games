pub mod create_game;
pub mod create_tribute;
pub mod fill_tributes_button;
pub mod game_actions_group;
pub mod game_day_log;
pub mod game_detail;
pub mod game_log;
pub mod game_play;
pub mod game_table;
pub mod game_table_row;
pub mod header;
pub mod home;
pub mod input_with_label;
pub mod tribute_actions_group;
pub mod tribute_box;
pub mod tribute_boxes;
pub mod tribute_detail;
pub mod tribute_list;
pub mod tribute_list_item;
pub mod tribute_table;
pub mod tribute_edit;

#[derive(Clone, Debug)]
pub struct ShowModal { pub show: bool }

#[derive(Clone, Debug)]
pub struct SelectedItem { pub id: i32 }

#[derive(Clone, Debug)]
pub struct UploadedFile {
    name: String,
    contents: Vec<u8>
}
