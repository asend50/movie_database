/*
By: <Asen Doiron>
Date: 2026-11-13
Program Details: <The purpose of this program is to allow the user to input a movie title along with its score and category into a database to have it stored there.>
*/

mod ui;
mod utils;
mod menu;
mod create;
mod delete;
mod update;
mod display;


use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "movie_database".to_string(),
        window_width: 1000,
        window_height: 1000,
        fullscreen: false,
        high_dpi: true,
        window_resizable: false,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "menu" => menu::run().await,
                "create" => create::run().await,
                "delete" => delete::run().await,
                "update" => update::run().await,
                "display" => display::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}

