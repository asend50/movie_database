use macroquad::prelude::*;
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;

pub async fn run() -> String {

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
        150.0,
        550.0,
        200.0,
        60.0,
        "Create",
        GREEN,
        YELLOW,
        30
    );

    let btn_delete = TextButton::new(
        400.0,
        550.0,
        200.0,
        60.0,
        "Delete",
        RED,
        ORANGE,
        30
    );

    let btn_update = TextButton::new(
        650.0,
        550.0,
        200.0,
        60.0,
        "Update",
        ORANGE,
        YELLOW,
        30
    );

    let btn_display = TextButton::new(
        400.0,
        650.0,
        200.0,
        60.0,
        "Display",
        ORANGE,
        YELLOW,
        30
    );

    loop {
        clear_background(BLUE);

        if btn_exit.click() {
            return "".to_string();
        }

        if btn_create.click() {
            return "create".to_string();
        }

        if btn_delete.click() {
            return "delete".to_string();
        }

        if btn_update.click() {
            return "update".to_string();
        }

        if btn_display.click() {
            return "display".to_string();
        }

        if is_key_pressed(KeyCode::Space) {
            return "screen2".to_string();
        }

        next_frame().await;
    }
}