use macroquad::prelude::*;
use crate::utils::database::{create_database_client, DatabaseTable};
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;

pub async fn run() -> String {

    let mut btn_create = TextButton::new(
        600.0,
        450.0,
        200.0,
        60.0,
        "Create",
        GREEN,
        YELLOW,
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

    let lbl_title = Label::new("Movie Title:", 200.0, 275.0, 50);
    let lbl_score = Label::new("Movie Score:", 200.0, 425.0, 50);
    let lbl_category = Label::new("Movie Category:", 200.0, 575.0, 50);
    let mut txt_title = TextInput::new(200.0, 300.0, 350.0, 55.0, 40.0);
    let mut txt_score = TextInput::new(200.0, 450.0, 350.0, 55.0, 40.0);
    let mut txt_category = TextInput::new(200.0, 600.0, 350.0, 55.0, 40.0);
    txt_title.set_max_chars(15);
    txt_score.set_max_chars(15);
    txt_category.set_max_chars(15);

    let client = create_database_client();
    loop {
        clear_background(DARKGRAY);

        if btn_exit.click() {
            return "".to_string();
        }

        if btn_return.click() {
            return "menu".to_string();
        }

        if !txt_title.get_text().trim().is_empty() &&
           !txt_score.get_text().trim().is_empty() &&
           !txt_category.get_text().trim().is_empty() {
            btn_create.enabled = true;
        } else {
            btn_create.enabled = false;
        }

        if btn_create.click() {
            let new_record = DatabaseTable {
                id: 0,
                title: txt_title.get_text().to_string(),
                score: txt_score.get_text().to_string(),
                category: txt_category.get_text().to_string(),
            };
            let insert_results = client.insert_record("movies", &new_record).await;
            if let Ok(id) = insert_results {
                // Inserted, id contains the new record's id
            } else {
                println!("Error inserting records from database: {} ", insert_results.err().unwrap());
            }

            let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("movies").await;
            if let Ok(result) = fetched_results {
                records = result;
            // To update a ListView with these records:
            // update_listview(&mut list_view, &records);
            } else {
                println!("Error fetching records from database: {} ", fetched_results.err().unwrap());
            }
        }

        lbl_title.draw();
        txt_title.draw();
        lbl_score.draw();
        txt_score.draw();
        lbl_category.draw();
        txt_category.draw();
        next_frame().await;
    }
}