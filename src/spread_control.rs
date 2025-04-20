use crate::cards::{Card, Deck};
use crate::GameState;
use macroquad::hash;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{get_frame_time, screen_width};
use macroquad::rand::gen_range;
use macroquad::ui::widgets::Editbox;
use macroquad::ui::{root_ui, Id, Skin};
use macroquad::window::screen_height;

pub fn render_spread(
    deck: &mut Deck,
    spread: &mut Vec<Card>,
    skin: &Skin,
    game_state: &mut GameState,
    is_popup_visible: &mut bool,
) {
    for c in deck.cards.iter_mut() {
        if c.get_position().is_none() {
            c.set_position(Vec2::ZERO);
        }
        c.update(!*is_popup_visible);
    }
    render_bottom_spread_menu(skin, game_state, is_popup_visible, deck);
    render_top_spread_menu(skin, deck);
    if *is_popup_visible {
        render_popup(skin, is_popup_visible, deck);
    }
}

pub fn render_bottom_spread_menu(
    skin: &Skin,
    game_state: &mut GameState,
    is_popup_visible: &mut bool,
    deck: &mut Deck,
) {
    root_ui().push_skin(skin);

    let win_dim = vec2(600.0, 75.0);
    let win_pos = vec2(
        screen_width() - win_dim.x - 20.0,
        screen_height() - win_dim.y - 25.0,
    );

    // Render menu
    root_ui().window(hash!(), win_pos, win_dim, |ui| {
        let button_size = ui.calc_size("Details") + 15.0;
        if ui.button(
            Some(vec2((button_size.x) / 2.0 - 25.0, 13.0)), // Center horizontally
            "Details",
        ) {
            *is_popup_visible = !*is_popup_visible;
        }

        let button_size = ui.calc_size("Shuffle") + 15.0;
        if ui.button(
            Some(vec2(button_size.x + 95.0, 13.0)), // Center horizontally
            "Shuffle",
        ) {
            deck.shuffle();
        }

        let button_size = ui.calc_size("Return") + 15.0;
        // Quit Button
        if ui.button(
            Some(vec2((win_dim.x - button_size.x) - 60.0, 12.0)), // Center horizontally
            "Return",
        ) {
            *game_state = GameState::Menu;
        }
    });

    root_ui().pop_skin();
}

pub fn render_top_spread_menu(skin: &Skin, deck: &mut Deck) {
    root_ui().push_skin(skin);

    let win_dim = vec2(400.0, 75.0);
    let win_pos = vec2(
        screen_width() - win_dim.x,
        win_dim.y-25.0,
    );

    // Render menu
    root_ui().window(hash!(), win_pos, win_dim, |ui| {
        let button_size = ui.calc_size("3-card") + 15.0;
        if ui.button(
            Some(vec2((button_size.x) / 2.0 - 25.0, 13.0)), // Center horizontally
            "3-card",
        ) {
            arrange_3_card(deck);
        }

        let button_size = ui.calc_size("Celtic") + 15.0;
        if ui.button(
            Some(vec2(button_size.x + 95.0, 13.0)), // Center horizontally
            "Celtic",
        ) {
            arrange_celtic(deck);
        }
    });

    root_ui().pop_skin();
}

pub fn arrange_celtic(deck: &mut Deck) {
    deck.shuffle();
    for (index, card) in deck.cards.iter_mut().enumerate() {
        if index == 10 { break; }
        match index {
            0 => {
                card.set_position(Vec2::new((screen_width() / 4.0) - card.get_dimensions().0/4.0, screen_height() / 2.0 - card.get_dimensions().1/4.0));
            }
            1 => {
                card.set_position(Vec2::new((screen_width() / 4.0) + 40.0, screen_height() / 2.0 - card.get_dimensions().1/4.0));
            }
            2 => {
                card.set_position(Vec2::new((screen_width() / 4.0) - card.get_dimensions().0/4.0, screen_height() / 2.0 - card.get_dimensions().1/4.0 + card.get_dimensions().1/2.0 + 5.0));
            }
            3 => {
                card.set_position(Vec2::new((screen_width() / 4.0) - card.get_dimensions().0/4.0 - card.get_dimensions().0/2.0 - 5.0, screen_height() / 2.0 - card.get_dimensions().1/4.0));
            }
            4 => {
                card.set_position(Vec2::new((screen_width() / 4.0) - card.get_dimensions().0/4.0, screen_height() / 2.0 - card.get_dimensions().1/4.0 - card.get_dimensions().1/2.0 - 5.0));
            }
            5 => {
                card.set_position(Vec2::new((screen_width() / 4.0) + card.get_dimensions().0/2.0 + 45.0, screen_height() / 2.0 - card.get_dimensions().1/4.0));
            }
            6 => {
                card.set_position(Vec2::new((3.0*screen_width() / 4.0) - card.get_dimensions().0/4.0, screen_height() / 2.0 - card.get_dimensions().1/4.0 + card.get_dimensions().1/2.0 + 5.0));
            }
            7 => {
                card.set_position(Vec2::new((3.0*screen_width() / 4.0) - card.get_dimensions().0/4.0, screen_height() / 2.0 - card.get_dimensions().1/4.0));
            }
            8 => {
                card.set_position(Vec2::new((3.0*screen_width() / 4.0) - card.get_dimensions().0/4.0, screen_height() / 2.0 - card.get_dimensions().1/4.0 - card.get_dimensions().1/2.0 - 5.0));
            }
            9 => {
                card.set_position(Vec2::new((3.0*screen_width() / 4.0) - card.get_dimensions().0/4.0 + card.get_dimensions().0/2.0 + 5.0, screen_height() / 2.0 - card.get_dimensions().1/4.0));
            }
            _ => {}
        }
    }

}

pub fn arrange_3_card(deck: &mut Deck) {
    deck.shuffle();
    for (index, card) in deck.cards.iter_mut().enumerate() {
        if index == 3 { break; }
        card.set_position(Vec2::new((screen_width() / 4.0)*(index as f32 + 1.0) - card.get_dimensions().0/4.0, screen_height() / 2.0 - card.get_dimensions().1/4.0));
    }
}

pub fn render_popup(skin: &Skin, is_popup_visible: &mut bool, deck: &mut Deck) {
    root_ui().push_skin(skin);
    let win_dim = vec2(1000.0, 700.0);
    let win_pos = vec2(
        (screen_width() / 2.0) - win_dim.x / 2.0,
        (screen_height() / 2.0) - win_dim.y / 2.0,
    );

    // Render menu
    root_ui().window(
        hash!(),
        win_pos, // Center the window itself
        win_dim,
        |ui| {
            let button_size = ui.calc_size("X") + 100.0;
            if ui.button(
                Some(vec2(win_dim.x - button_size.x, 10.0)), // Center horizontally
                "X",
            ) {
                *is_popup_visible = !*is_popup_visible;
            }
            // Measure text size to determine its width and height
            let label_size = ui.calc_size("Tarot Card Game");
            ui.label(
                Some(vec2(win_dim.x / 2.0 - label_size.x + 100.0, 15.0)), // Dynamically center horizontally
                "Tarot Cards Meanings",
            );

            let mut str: String = "".to_string();
            for card in &deck.cards {
                if card.is_face_up() {
                    str = format!("{}\n{}", str, card.get_meaning());
                }
            }
            str = format!("{}\n\n", str);
            let mut x = Editbox::new(
                macroquad::ui::Id::default(),
                vec2(win_dim.x - 50.0, win_dim.y - 30.0),
            );
            x.position(Vec2::new(win_pos.x + 20.0, win_pos.y + 80.0))
                .ui(ui, &mut str);
        },
    );

    root_ui().pop_skin();
}
