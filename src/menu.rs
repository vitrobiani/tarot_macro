use crate::cards::{Card, Deck};
use crate::GameState;
use macroquad::prelude::*;
use macroquad::rand::rand;
use macroquad::ui::{hash, root_ui, widgets, Skin};
use std::ops::Deref;

pub fn render_menu(skin: &Skin, game_state: &mut GameState) {
    root_ui().push_skin(skin);

    // Render menu
    root_ui().window(
        hash!(),
        vec2(
            (screen_width() / 2.0) - 300.0,
            (screen_height() / 2.0) - 150.0,
        ), // Center the window itself
        vec2(600.0, 300.0),
        |ui| {
            // Measure text size to determine its width and height
            let label_size = ui.calc_size("Tarot Card Game");
            ui.label(
                Some(vec2((600.0 - label_size.x) / 2.0, 50.0)), // Dynamically center horizontally
                "Tarot Card Game",
            );

            // Dynamically position buttons
            let button_size = ui.calc_size("Start Game") + 130.0;
            // Start Game Button
            if ui.button(
                Some(vec2((600.0 - button_size.x) / 2.0, 100.0)), // Center horizontally
                "Start Game",
            ) {
                *game_state = GameState::Spread;
            }
            let button_size = ui.calc_size("Quit") + 120.0;
            // Quit Button
            if ui.button(
                Some(vec2((600.0 - button_size.x) / 2.0, 170.0)), // Center horizontally
                "Quit",
            ) {
                std::process::exit(0);
            }
        },
    );

    root_ui().pop_skin();
}
