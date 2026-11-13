use macroquad::prelude::*;
use crate::utils::database::{create_database_client, DatabaseTable};
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;

pub async fn run() -> String {

    let mut btn_delete = TextButton::new(
        600.0,
        450.0,
        200.0,
        60.0,
        "Delete",
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
            btn_delete.enabled = true;
        } else {
            btn_delete.enabled = false;
        }

        if btn_delete.click() {
            let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("movies").await;
            if let Ok(result) = fetched_results {
                records = result;
            }
            for i in &records {
                println!("ID: {}, Title: {}, Score: {}, Category: {}", i.id, i.title, i.score, i.category);
                if txt_title.get_text() == i.title && txt_score.get_text() == i.score && txt_category.get_text() == i.category {
                    if let Ok(deleted_count) = client.delete_record_by_id("movies", i.id as i64).await {
                        // deleted_count is the number of records deleted
                    } else {
                        // Handle error
                    }
                }
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