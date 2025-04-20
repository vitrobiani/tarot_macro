#[allow(
    unused_imports,
    dead_code,
    unused_variables,
    unused_mut,
    unused_macros,
    unused
)]
pub mod cards;
mod classic_skin;
mod menu;
mod spread_control;

use crate::cards::{Card, Deck};
use macroquad::prelude::*;

enum GameState {
    Loading,
    Menu,
    Spread,
}

fn window_conf() -> Conf {
    Conf {
        window_width: 1200,
        window_height: 800,
        window_title: "Tarot".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut skin = Option::default();
    let mut skin_smaller = Option::default();

    let mut game_state = GameState::Loading;
    let mut d: Deck = Deck::new();
    let mut spread: Vec<Card> = Vec::new();
    let mut is_popup_visible = false;

    loop {
        clear_background(GRAY);

        match game_state {
            GameState::Loading => {
                draw_text(
                    "Loading...",
                    screen_width() / 2.0,
                    screen_height() / 2.0,
                    40.0,
                    DARKGRAY,
                );
                if d.cards.is_empty() {
                    next_frame().await;
                }
                skin = Option::Some(classic_skin::init());
                skin_smaller = Option::Some(classic_skin::init_smaller());
                if let _ = d.fill_deck().await {
                    d.shuffle();
                    game_state = GameState::Menu;
                }
            }
            GameState::Menu => {
                let skin = match skin {
                    Some(ref mut skin) => skin,
                    None => return,
                };
                menu::render_menu(&skin, &mut game_state);
            }
            GameState::Spread => {
                let skin_smaller = match skin_smaller {
                    Some(ref mut skin_smaller) => skin_smaller,
                    None => return,
                };
                spread_control::render_spread(
                    &mut d,
                    &mut spread,
                    &skin_smaller,
                    &mut game_state,
                    &mut is_popup_visible,
                );
            }
        }

        next_frame().await;
    }
}
