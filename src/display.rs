use macroquad::prelude::*;
use crate::utils::database::{create_database_client, DatabaseTable};
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::ui::listview::ListView;

pub async fn run() -> String {

    let mut btn_search = TextButton::new(
        700.0,
        110.0,
        100.0,
        60.0,
        "Search",
        RED,
        ORANGE,
        30
    );

    let btn_return = TextButton::new(
        25.0,
        25.0,
        60.0,
        60.0,
        "<-",
        BLUE,
        GREEN,
        30
    );

    let btn_exit = TextButton::new(
        925.0,
        25.0,
        60.0,
        60.0,
        "X",
        RED,
        ORANGE,
        30
    );

    let lbl_titles = Label::new("Movie Titles:", 100.0, 275.0, 50);
    let mut lbl_list = Label::new("", 100.0, 350.0, 30);
    let lbl_search = Label::new("Search:", 150.0, 150.0, 50);
    let mut txt_search = TextInput::new(325.0, 110.0, 350.0, 55.0, 40.0);
    txt_search.set_max_chars(19);
    lbl_list.with_fixed_size(800.0, 500.0);

    let client = create_database_client();

    let mut items = vec![];
    let mut list_view = ListView::new(&items, 100.0, 350.0, 30);

    loop {
        clear_background(DARKGRAY);

        if btn_exit.click() {
            return "".to_string();
        }

        if btn_return.click() {
            return "menu".to_string();
        }


        if !txt_search.get_text().trim().is_empty() {
            btn_search.enabled = true;
        } else {
            btn_search.enabled = false;
        }

        if btn_search.click() {
            let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("movies").await;
            if let Ok(result) = fetched_results {
                records = result;
            }
    for i in 0..records.len() {
        items.push(records[i]);
            list_view.clear();
            list_view.add_items(&items);
    lbl_list.set_text(&format!("{:?}\n", items));
    }}
    
        lbl_titles.draw();
        lbl_search.draw();
        txt_search.draw();
        list_view.draw();

        next_frame().await;
    }
}