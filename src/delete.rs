use macroquad::prelude::*;
use crate::utils::database::{create_database_client, DatabaseTable};
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: TextureManager) -> (String, TextureManager) {

    let delete= StillImage::from_preload(
    tm.get_preload("assets/delete.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

    let mut btn_delete = TextButton::new(
        600.0,
        550.0,
        200.0,
        60.0,
        "Delete",
        BLACK,
        GRAY,
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

    let lbl_title = Label::new("Movie Title:", 200.0, 400.0, 50);
    let lbl_score = Label::new("Movie Score:", 200.0, 525.0, 50);
    let lbl_category = Label::new("Movie Category:", 200.0, 650.0, 50);
    let mut txt_title = TextInput::new(200.0, 425.0, 350.0, 55.0, 40.0);
    let mut txt_score = TextInput::new(200.0, 550.0, 350.0, 55.0, 40.0);
    txt_score.set_allowed_chars("0123456789.");
    let mut txt_category = TextInput::new(200.0, 675.0, 350.0, 55.0, 40.0);
    txt_title.set_max_chars(18);
    txt_score.set_max_chars(3);
    txt_category.set_max_chars(5);

    let client = create_database_client();
    loop {
        clear_background(WHITE);
        delete.draw();

        if btn_exit.click() {
            return ("".to_string(), tm);
        }

        if btn_return.click() {
            return ("menu".to_string(), tm);
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