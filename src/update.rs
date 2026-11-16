use macroquad::prelude::*;
use crate::utils::database::{create_database_client, DatabaseTable};
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;

pub async fn run() -> String {

    let mut btn_update = TextButton::new(
        400.0,
        700.0,
        200.0,
        60.0,
        "Update",
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

    let lbl_title = Label::new("Old Movie Title:", 100.0, 275.0, 40);
    let lbl_score = Label::new("Old Movie Score:", 100.0, 425.0, 40);
    let lbl_category = Label::new("Old Movie Category:", 100.0, 575.0, 40);
    let lbl_updatetitle = Label::new("New Movie Title:", 550.0, 275.0, 40);
    let lbl_updatescore = Label::new("New Movie Score:", 550.0, 425.0, 40);
    let lbl_updatecategory = Label::new("New Movie Category:", 550.0, 575.0, 40);
    let mut txt_title = TextInput::new(100.0, 300.0, 350.0, 55.0, 40.0);
    let mut txt_score = TextInput::new(100.0, 450.0, 350.0, 55.0, 40.0);
    let mut txt_category = TextInput::new(100.0, 600.0, 350.0, 55.0, 40.0);
    let mut txt_updatetitle = TextInput::new(550.0, 300.0, 350.0, 55.0, 40.0);
    let mut txt_updatescore = TextInput::new(550.0, 450.0, 350.0, 55.0, 40.0);
    let mut txt_updatecategory = TextInput::new(550.0, 600.0, 350.0, 55.0, 40.0);
    txt_title.set_max_chars(19);
    txt_score.set_max_chars(19);
    txt_category.set_max_chars(19);
    txt_updatetitle.set_max_chars(19);
    txt_updatescore.set_max_chars(19);
    txt_updatecategory.set_max_chars(19);
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
           !txt_category.get_text().trim().is_empty() &&
           !txt_updatetitle.get_text().trim().is_empty() &&
           !txt_updatescore.get_text().trim().is_empty() &&
           !txt_updatecategory.get_text().trim().is_empty() {
            btn_update.enabled = true;
        } else {
            btn_update.enabled = false;
        }

        if btn_update.click() {
            let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("movies").await;
            if let Ok(result) = fetched_results {
                records = result;
            }
            for i in &records {
                if txt_title.get_text() == i.title && txt_score.get_text() == i.score && txt_category.get_text() == i.category {
                    if let Ok(updated_count) = client.update_record_by_id("movies", i.id as i64, "title", &txt_updatetitle.get_text()).await {
                        // updated_count is the number of records updated
                    } else {
                        // Handle error
                    }
                    if let Ok(updated_count) = client.update_record_by_id("movies", i.id as i64, "score", &txt_updatescore.get_text()).await {
                        // updated_count is the number of records updated
                    } else {
                        // Handle error
                    }
                    if let Ok(updated_count) = client.update_record_by_id("movies", i.id as i64, "category", &txt_updatecategory.get_text()).await {
                        // updated_count is the number of records updated
                    } else {
                        // Handle error
                    }
                }
            }
        }

        lbl_title.draw();
        lbl_score.draw();
        lbl_category.draw();
        lbl_updatetitle.draw();
        lbl_updatescore.draw();
        lbl_updatecategory.draw();
        txt_title.draw();
        txt_score.draw();
        txt_category.draw();
        txt_updatetitle.draw();
        txt_updatescore.draw();
        txt_updatecategory.draw();
        next_frame().await;
    }
}