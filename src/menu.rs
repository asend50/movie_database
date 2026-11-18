use macroquad::prelude::*;
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: TextureManager) -> (String, TextureManager) {

    let menu= StillImage::from_preload(
    tm.get_preload("assets/menu.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
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

    let btn_create = TextButton::new(
        625.0,
        425.0,
        155.0,
        60.0,
        "Create",
        BLACK,
        GRAY,
        50
    );

    let btn_delete = TextButton::new(
        625.0,
        525.0,
        155.0,
        60.0,
        "Delete",
        BLACK,
        GRAY,
        50
    );

    let btn_update = TextButton::new(
        800.0,
        425.0,
        155.0,
        60.0,
        "Update",
        BLACK,
        GRAY,
        50
    );

    let btn_display = TextButton::new(
        800.0,
        525.0,
        155.0,
        60.0,
        "Display",
        BLACK,
        GRAY,
        50
    );

    loop {
        clear_background(WHITE);
        menu.draw();

        if btn_exit.click() {
            return ("".to_string(), tm);
        }

        if btn_create.click() {
            return ("create".to_string(), tm);
        }

        if btn_delete.click() {
            return ("delete".to_string(), tm);
        }

        if btn_update.click() {
            return ("update".to_string(), tm);
        }

        if btn_display.click() {
            return ("display".to_string(), tm);
        }

        if is_key_pressed(KeyCode::Space) {
            return ("screen2".to_string(), tm);
        }

        next_frame().await;
    }
}