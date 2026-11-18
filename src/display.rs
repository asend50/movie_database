use macroquad::prelude::*;
use crate::utils::database::{create_database_client, DatabaseTable};
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::ui::listview::ListView;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: TextureManager) -> (String, TextureManager) {

    let search= StillImage::from_preload(
    tm.get_preload("assets/search.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

    let mut btn_search = TextButton::new(
        700.0,
        360.0,
        100.0,
        60.0,
        "Search",
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

    let lbl_titles = Label::new("Movie Titles:", 100.0, 475.0, 70);
    let lbl_sections = Label::new("| Title: | Score: | Category: |", 100.0, 525.0, 40);
    let lbl_search = Label::new("Search:", 150.0, 400.0, 50);
    let mut txt_search = TextInput::new(325.0, 360.0, 350.0, 55.0, 30.0);
    txt_search.set_max_chars(19);

    let client = create_database_client();

    let mut items = vec![];
    let mut list_view = ListView::new(&items, 100.0, 575.0, 35);

    let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("movies").await;
            if let Ok(result) = fetched_results {
                records = result;
            }
        for record in &records {
                items.push(format!(
                "| {} | {} | {} |", record.title, record.score, record.category));
                list_view.clear();
                list_view.add_items(&items);
            }

    loop {
        clear_background(WHITE);
        search.draw();

        if btn_exit.click() {
            return ("".to_string(), tm);
        }

        if btn_return.click() {
            return ("menu".to_string(), tm);
        }
        

        if btn_search.click() {
            list_view.clear();
            items.clear();
            let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("movies").await;
            if let Ok(result) = fetched_results {
                records = result;
            }
            let search = txt_search.get_text().trim().to_lowercase();
            if search.is_empty() {
                for record in &records {
                    items.push(format!(
                    "| {} | {} | {} |", record.title, record.score, record.category));
                    list_view.clear();
                    list_view.add_items(&items);
                }
            } else {
                for record in &records {
                if record.title.to_lowercase().contains(&search) {
                    let searchtitle = txt_search.get_text().to_string();
                    items.push(format!(
                    "| {} | {} | {} |", record.title, record.score, record.category));
                    list_view.clear();
                    list_view.add_items(&items);
                }
            }
        }
    }
    
        lbl_titles.draw();
        lbl_sections.draw();
        lbl_search.draw();
        txt_search.draw();
        list_view.draw();

        next_frame().await;
    }
}