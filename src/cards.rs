use crate::Texture2D;
#[allow(
    unused_imports,
    dead_code,
    unused_variables,
    unused_mut,
    unused_macros,
    unused
)]
use std::fmt::format;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
// use macroquad::miniquad::TextureKind::Texture2D;

use macroquad::prelude::*;
use macroquad::rand::*;
use textwrap::fill;

#[derive(Debug, Clone)]
pub enum Arcana {
    Minor,
    Major,
}

#[derive(Debug, Clone)]
pub enum MajorArcana {
    Fool,
    Magician,
    HighPriestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Strength,
    Hermit,
    TheWheelOfFortune,
    Justice,
    HangedMan,
    Death,
    Temperence,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    World,
}

impl MajorArcana {
    fn iterator() -> impl Iterator<Item = MajorArcana> {
        [
            MajorArcana::Fool,
            MajorArcana::Magician,
            MajorArcana::HighPriestess,
            MajorArcana::Empress,
            MajorArcana::Emperor,
            MajorArcana::Hierophant,
            MajorArcana::Lovers,
            MajorArcana::Chariot,
            MajorArcana::Strength,
            MajorArcana::Hermit,
            MajorArcana::TheWheelOfFortune,
            MajorArcana::Justice,
            MajorArcana::HangedMan,
            MajorArcana::Death,
            MajorArcana::Temperence,
            MajorArcana::Devil,
            MajorArcana::Tower,
            MajorArcana::Star,
            MajorArcana::Moon,
            MajorArcana::Sun,
            MajorArcana::Judgement,
            MajorArcana::World,
        ]
        .iter()
        .cloned()
    }
}

#[derive(Debug, Clone)]
pub enum MinorArcana {
    Wands,
    Cups,
    Swords,
    Pentacles,
}

impl MinorArcana {
    pub fn iterator() -> impl Iterator<Item = MinorArcana> {
        [
            MinorArcana::Wands,
            MinorArcana::Cups,
            MinorArcana::Swords,
            MinorArcana::Pentacles,
        ]
        .iter()
        .cloned()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NumericalValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Page,
    Knight,
    Queen,
    King,
}

impl NumericalValue {
    fn iterator() -> impl Iterator<Item = NumericalValue> {
        [
            NumericalValue::Ace,
            NumericalValue::Two,
            NumericalValue::Three,
            NumericalValue::Four,
            NumericalValue::Five,
            NumericalValue::Six,
            NumericalValue::Seven,
            NumericalValue::Eight,
            NumericalValue::Nine,
            NumericalValue::Ten,
            NumericalValue::Page,
            NumericalValue::Knight,
            NumericalValue::Queen,
            NumericalValue::King,
        ]
        .iter()
        .cloned()
    }
}

impl Symbol {
    async fn meaning(&self) -> String {
        match self {
            Symbol::Major(major_arcana) => {
                let meaning_path = format!("assets/Meanings/major_arcana/{}", Self::match_major_meaning(major_arcana));
                let major_error_msg = format!("problem reading meaning from {}", meaning_path);
                let file_meaning = load_string(meaning_path.as_str()).await.expect(&major_error_msg);
                let major_meaning = format!(" + Major Arcana: {}\n{}\n", Self::major_arcana_string(major_arcana), file_meaning);
                fill(&major_meaning, 76)
            },
            Symbol::Minor(suit, numer) => {
                let numer_meaning = Self::match_numer_meaning(numer);
                let suit_meaning = Self::match_suit_meaning(suit);
                let numer_meaning_path = format!("assets/Meanings/minor_arcana/{}", numer_meaning);
                let suit_meaning_path = format!("assets/Meanings/minor_arcana/{}", suit_meaning);

                let numer_error_msg = format!("problem reading numerical meaning in {}", numer_meaning_path);
                let suit_error_msg = format!("problem reading suit meaning in {}", suit_meaning_path);

                let numer_meaning = load_string(numer_meaning_path.as_str()).await.expect(&numer_error_msg);
                let suit_meaning = load_string(suit_meaning_path.as_str()).await.expect(&suit_error_msg);

                let meaning = format!(" - The {} of {}:\nNumerical meaning: {}\nArcana meaning: {}\n", Self::numerical_value_string(numer), Self::suit_string(suit), numer_meaning, suit_meaning);
                fill(&meaning, 76)
            },
        }
    }

    fn major_arcana_string(arc: &MajorArcana) -> String {
        match arc {
            MajorArcana::Fool => "The Fool".to_string(),
            MajorArcana::Magician => "The Magician".to_string(),
            MajorArcana::HighPriestess => "The High Priestess".to_string(),
            MajorArcana::Empress => "The Empress".to_string(),
            MajorArcana::Emperor => "The Emperor".to_string(),
            MajorArcana::Hierophant => "The Hierophant".to_string(),
            MajorArcana::Lovers => "The Lovers".to_string(),
            MajorArcana::Chariot => "The Chariot".to_string(),
            MajorArcana::Strength => "Strength".to_string(),
            MajorArcana::Hermit => "The Hermit".to_string(),
            MajorArcana::TheWheelOfFortune => "Wheel Of Fortune".to_string(),
            MajorArcana::Justice => "Justice".to_string(),
            MajorArcana::HangedMan => "The Hanged Man".to_string(),
            MajorArcana::Death => "Death".to_string(),
            MajorArcana::Temperence => "Temperance".to_string(),
            MajorArcana::Devil => "The Devil".to_string(),
            MajorArcana::Tower => "The Tower".to_string(),
            MajorArcana::Star => "The Star".to_string(),
            MajorArcana::Moon => "The Moon".to_string(),
            MajorArcana::Sun => "The Sun".to_string(),
            MajorArcana::Judgement => "Judgement".to_string(),
            MajorArcana::World => "The World".to_string(),
        }
    }
    fn numerical_value_string(minor: &NumericalValue) -> String {
        match minor {
            NumericalValue::Ace => "Ace".to_string(),
            NumericalValue::Two => "Two".to_string(),
            NumericalValue::Three => "Three".to_string(),
            NumericalValue::Four => "Four".to_string(),
            NumericalValue::Five => "Five".to_string(),
            NumericalValue::Six => "Six".to_string(),
            NumericalValue::Seven => "Seven".to_string(),
            NumericalValue::Eight => "Eight".to_string(),
            NumericalValue::Nine => "Nine".to_string(),
            NumericalValue::Ten => "Ten".to_string(),
            NumericalValue::Page => "Page".to_string(),
            NumericalValue::Knight => "Knight".to_string(),
            NumericalValue::Queen => "Queen".to_string(),
            NumericalValue::King => "King".to_string(),
        }
    }

    pub fn suit_string(minor: &MinorArcana) -> String {
        match minor {
            MinorArcana::Wands => "Wands".to_string(),
            MinorArcana::Pentacles => "Pentacles".to_string(),
            MinorArcana::Swords => "Swords".to_string(),
            MinorArcana::Cups => "Cups".to_string(),
        }
    }

    fn match_major_meaning(arc: &MajorArcana) -> String {
        match arc {
            MajorArcana::Fool => "00-TheFool.txt".to_string(),
            MajorArcana::Magician => "01-TheMagician.txt".to_string(),
            MajorArcana::HighPriestess => "02-TheHighPriestess.txt".to_string(),
            MajorArcana::Empress => "03-TheEmpress.txt".to_string(),
            MajorArcana::Emperor => "04-TheEmperor.txt".to_string(),
            MajorArcana::Hierophant => "05-TheHierophant.txt".to_string(),
            MajorArcana::Lovers => "06-TheLovers.txt".to_string(),
            MajorArcana::Chariot => "07-TheChariot.txt".to_string(),
            MajorArcana::Strength => "08-Strength.txt".to_string(),
            MajorArcana::Hermit => "09-TheHermit.txt".to_string(),
            MajorArcana::TheWheelOfFortune => "10-WheelOfFortune.txt".to_string(),
            MajorArcana::Justice => "11-Justice.txt".to_string(),
            MajorArcana::HangedMan => "12-TheHangedMan.txt".to_string(),
            MajorArcana::Death => "13-Death.txt".to_string(),
            MajorArcana::Temperence => "14-Temperance.txt".to_string(),
            MajorArcana::Devil => "15-TheDevil.txt".to_string(),
            MajorArcana::Tower => "16-TheTower.txt".to_string(),
            MajorArcana::Star => "17-TheStar.txt".to_string(),
            MajorArcana::Moon => "18-TheMoon.txt".to_string(),
            MajorArcana::Sun => "19-TheSun.txt".to_string(),
            MajorArcana::Judgement => "20-Judgement.txt".to_string(),
            MajorArcana::World => "21-TheWorld.txt".to_string(),
        }
    }

    pub fn match_numer_meaning(minor: &NumericalValue) -> String {
        match minor {
            NumericalValue::Ace => "01.txt".to_string(),
            NumericalValue::Two => "02.txt".to_string(),
            NumericalValue::Three => "03.txt".to_string(),
            NumericalValue::Four => "04.txt".to_string(),
            NumericalValue::Five => "05.txt".to_string(),
            NumericalValue::Six => "06.txt".to_string(),
            NumericalValue::Seven => "07.txt".to_string(),
            NumericalValue::Eight => "08.txt".to_string(),
            NumericalValue::Nine => "09.txt".to_string(),
            NumericalValue::Ten => "10.txt".to_string(),
            NumericalValue::Page => "11.txt".to_string(),
            NumericalValue::Knight => "12.txt".to_string(),
            NumericalValue::Queen => "13.txt".to_string(),
            NumericalValue::King => "14.txt".to_string(),
        }
    }

    pub fn match_suit_meaning(minor: &MinorArcana) -> String {
        match minor {
            MinorArcana::Wands => "Wands.txt".to_string(),
            MinorArcana::Pentacles => "Pentacles.txt".to_string(),
            MinorArcana::Swords => "Swords.txt".to_string(),
            MinorArcana::Cups => "Cups.txt".to_string(),
        }
    }

    fn front(&self) -> String {
        match self {
            Symbol::Major(major_arcana) => Symbol::match_major_texture(major_arcana),
            Symbol::Minor(suit, numer) => match suit {
                MinorArcana::Wands => Symbol::match_wands_texture(numer),
                MinorArcana::Cups => Symbol::match_cups_texture(numer),
                MinorArcana::Swords => Symbol::match_swords_texture(numer),
                MinorArcana::Pentacles => Symbol::match_pentacles_texture(numer),
            },
        }
    }

    fn match_major_texture(arc: &MajorArcana) -> String {
        match arc {
            MajorArcana::Fool => "00-TheFool.png".to_string(),
            MajorArcana::Magician => "01-TheMagician.png".to_string(),
            MajorArcana::HighPriestess => "02-TheHighPriestess.png".to_string(),
            MajorArcana::Empress => "03-TheEmpress.png".to_string(),
            MajorArcana::Emperor => "04-TheEmperor.png".to_string(),
            MajorArcana::Hierophant => "05-TheHierophant.png".to_string(),
            MajorArcana::Lovers => "06-TheLovers.png".to_string(),
            MajorArcana::Chariot => "07-TheChariot.png".to_string(),
            MajorArcana::Strength => "08-Strength.png".to_string(),
            MajorArcana::Hermit => "09-TheHermit.png".to_string(),
            MajorArcana::TheWheelOfFortune => "10-WheelOfFortune.png".to_string(),
            MajorArcana::Justice => "11-Justice.png".to_string(),
            MajorArcana::HangedMan => "12-TheHangedMan.png".to_string(),
            MajorArcana::Death => "13-Death.png".to_string(),
            MajorArcana::Temperence => "14-Temperance.png".to_string(),
            MajorArcana::Devil => "15-TheDevil.png".to_string(),
            MajorArcana::Tower => "16-TheTower.png".to_string(),
            MajorArcana::Star => "17-TheStar.png".to_string(),
            MajorArcana::Moon => "18-TheMoon.png".to_string(),
            MajorArcana::Sun => "19-TheSun.png".to_string(),
            MajorArcana::Judgement => "20-Judgement.png".to_string(),
            MajorArcana::World => "21-TheWorld.png".to_string(),
        }
    }

    pub fn match_wands_texture(minor: &NumericalValue) -> String {
        match minor {
            NumericalValue::Ace => "Wands01.png".to_string(),
            NumericalValue::Two => "Wands02.png".to_string(),
            NumericalValue::Three => "Wands03.png".to_string(),
            NumericalValue::Four => "Wands04.png".to_string(),
            NumericalValue::Five => "Wands05.png".to_string(),
            NumericalValue::Six => "Wands06.png".to_string(),
            NumericalValue::Seven => "Wands07.png".to_string(),
            NumericalValue::Eight => "Wands08.png".to_string(),
            NumericalValue::Nine => "Wands09.png".to_string(),
            NumericalValue::Ten => "Wands10.png".to_string(),
            NumericalValue::Page => "Wands11.png".to_string(),
            NumericalValue::Knight => "Wands12.png".to_string(),
            NumericalValue::Queen => "Wands13.png".to_string(),
            NumericalValue::King => "Wands14.png".to_string(),
        }
    }

    pub fn match_cups_texture(minor: &NumericalValue) -> String {
        match minor {
            NumericalValue::Ace => "Cups01.png".to_string(),
            NumericalValue::Two => "Cups02.png".to_string(),
            NumericalValue::Three => "Cups03.png".to_string(),
            NumericalValue::Four => "Cups04.png".to_string(),
            NumericalValue::Five => "Cups05.png".to_string(),
            NumericalValue::Six => "Cups06.png".to_string(),
            NumericalValue::Seven => "Cups07.png".to_string(),
            NumericalValue::Eight => "Cups08.png".to_string(),
            NumericalValue::Nine => "Cups09.png".to_string(),
            NumericalValue::Ten => "Cups10.png".to_string(),
            NumericalValue::Page => "Cups11.png".to_string(),
            NumericalValue::Knight => "Cups12.png".to_string(),
            NumericalValue::Queen => "Cups13.png".to_string(),
            NumericalValue::King => "Cups14.png".to_string(),
        }
    }

    pub fn match_swords_texture(minor: &NumericalValue) -> String {
        match minor {
            NumericalValue::Ace => "Swords01.png".to_string(),
            NumericalValue::Two => "Swords02.png".to_string(),
            NumericalValue::Three => "Swords03.png".to_string(),
            NumericalValue::Four => "Swords04.png".to_string(),
            NumericalValue::Five => "Swords05.png".to_string(),
            NumericalValue::Six => "Swords06.png".to_string(),
            NumericalValue::Seven => "Swords07.png".to_string(),
            NumericalValue::Eight => "Swords08.png".to_string(),
            NumericalValue::Nine => "Swords09.png".to_string(),
            NumericalValue::Ten => "Swords10.png".to_string(),
            NumericalValue::Page => "Swords11.png".to_string(),
            NumericalValue::Knight => "Swords12.png".to_string(),
            NumericalValue::Queen => "Swords13.png".to_string(),
            NumericalValue::King => "Swords14.png".to_string(),
        }
    }

    pub fn match_pentacles_texture(minor: &NumericalValue) -> String {
        match minor {
            NumericalValue::Ace => "Pentacles01.png".to_string(),
            NumericalValue::Two => "Pentacles02.png".to_string(),
            NumericalValue::Three => "Pentacles03.png".to_string(),
            NumericalValue::Four => "Pentacles04.png".to_string(),
            NumericalValue::Five => "Pentacles05.png".to_string(),
            NumericalValue::Six => "Pentacles06.png".to_string(),
            NumericalValue::Seven => "Pentacles07.png".to_string(),
            NumericalValue::Eight => "Pentacles08.png".to_string(),
            NumericalValue::Nine => "Pentacles09.png".to_string(),
            NumericalValue::Ten => "Pentacles10.png".to_string(),
            NumericalValue::Page => "Pentacles11.png".to_string(),
            NumericalValue::Knight => "Pentacles12.png".to_string(),
            NumericalValue::Queen => "Pentacles13.png".to_string(),
            NumericalValue::King => "Pentacles14.png".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Major(MajorArcana),
    Minor(MinorArcana, NumericalValue),
}

#[derive(Debug, Clone)]
pub struct Card {
    card_arcana: Arcana,
    card_symbol: Symbol,
    meaning: String,
    upsidedown_meaning: String,
    card_front: String,
    card_back: String,
    card_front_texture: Texture2D,
    card_back_texture: Texture2D,
    card_width: f32,
    card_height: f32,
    scaler: f32,
    card_type: String,
    position: Option<Vec2>,
    is_dragging: bool,
    drag_timer: f32,
    is_face_up: bool,
}

static mut DRAG: bool = false;

impl Card {
    pub async fn new(arc: Arcana, symb: Symbol, ct: String) -> Self {
        let front: String = format!("assets/{}/{}", ct, symb.front());
        let front_texture = load_texture(front.as_str()).await.unwrap();
        let back_texture = load_texture("assets/Classic/CardBacks.png").await.unwrap();
        let cw = front_texture.width();
        let ch = front_texture.height();
        Card {
            card_arcana: match symb {
                Symbol::Major(_) => Arcana::Major,
                Symbol::Minor(_, _) => Arcana::Minor,
            },
            card_symbol: symb.clone(),
            card_back: "assets/Classic/CardBacks.png".to_string(),
            card_front: front,
            card_front_texture: front_texture,
            card_back_texture: back_texture,
            card_width: cw,
            card_height: ch,
            card_type: ct,
            meaning: symb.meaning().await,
            upsidedown_meaning: "".to_string(),
            position: None,
            is_dragging: false,
            drag_timer: 0.0,
            is_face_up: false,
            scaler: 0.5,
        }
    }
    pub async fn new_with_pos(arc: Arcana, symb: Symbol, ct: String, pos: Vec2) -> Self {
        let front: String = format!("assets/{}/{}", ct, symb.front());
        let front_texture = load_texture(front.as_str()).await.unwrap();
        let back_texture = load_texture("assets/Classic/CardBacks.png").await.unwrap();
        let cw = front_texture.width();
        let ch = front_texture.height();
        Card {
            card_arcana: match symb {
                Symbol::Major(_) => Arcana::Major,
                Symbol::Minor(_, _) => Arcana::Minor,
            },
            card_symbol: symb,
            card_front_texture: load_texture(&front.as_str()).await.unwrap(),
            card_back_texture: load_texture("assets/Classic/CardBacks.png".to_string().as_str())
                .await
                .unwrap(),
            card_front: front,
            card_back: "assets/Classic/CardBacks.png".to_string(),
            card_width: cw,
            card_height: ch,
            card_type: ct,
            meaning: "".to_string(),
            upsidedown_meaning: "".to_string(),
            position: Some(pos),
            is_dragging: false,
            drag_timer: 0.0,
            is_face_up: false,
            scaler: 0.5,
        }
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.position = Some(pos);
    }

    pub fn get_position(&self) -> Option<Vec2> {
        self.position
    }

    pub fn set_scaler(&mut self, scaler: f32) {
        self.scaler = scaler;
    }

    pub fn get_meaning(&self) -> String {
        self.meaning.clone()
    }

    pub fn is_face_up(&self) -> bool {
        self.is_face_up
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.card_width, self.card_height)
    }

    pub fn front(self) -> String {
        self.card_front
    }

    pub fn back(self) -> String {
        self.card_back
    }

    pub fn draw(&self) {
        let msg = "No position was set to a card about to be shown";
        draw_texture_ex(
            if self.is_face_up {
                &self.card_front_texture
            } else {
                &self.card_back_texture
            },
            self.position.expect(msg).x,
            self.position.expect(msg).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    (if self.is_face_up {
                        self.card_front_texture.width()
                    } else {
                        self.card_back_texture.width()
                    }) * self.scaler,
                    (if self.is_face_up {
                        self.card_front_texture.height()
                    } else {
                        self.card_back_texture.height()
                    }) * self.scaler,
                )),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, allow_dragging: bool) {
        let dt = get_frame_time();
        if allow_dragging {
            self.flip_card();
            self.drag(dt);
        }
        self.draw();
    }

    pub fn flip_card(&mut self) {
        let mouse_pos = mouse_position().into();
        if is_mouse_button_released(MouseButton::Left)
            && self.is_mouse_over(mouse_pos)
            && !self.is_dragging
            && unsafe { !DRAG }
        {
            self.is_face_up = !self.is_face_up
        }
    }

    pub fn drag(&mut self, dt: f32) {
        let mouse_pos = mouse_position().into();

        if is_mouse_button_down(MouseButton::Left) {
            // Increase timer while button is held
            self.drag_timer += dt;

            // Start dragging if button was held long enough
            if self.drag_timer > 0.2
                && !self.is_dragging
                && self.is_mouse_over(mouse_pos)
                && unsafe { !DRAG }
            {
                self.is_dragging = true;
                unsafe {
                    DRAG = true;
                }
            }
        } else {
            // Reset timer and stop dragging when mouse is released
            self.drag_timer = 0.0;
            self.is_dragging = false;
            unsafe {
                DRAG = false;
            }
        }

        // Move the card if it's being dragged
        if self.is_dragging {
            self.position =
                Some(mouse_pos - Vec2::new(self.card_width / 2.0, self.card_height / 2.0));
        }
    }

    fn is_mouse_over(&self, mouse_pos: Vec2) -> bool {
        let msg =
            "No position was set to a card about to be shown when checking if mouse is over a card";
        mouse_pos.x >= self.position.expect(msg).x
            && mouse_pos.x <= self.position.expect(msg).x + self.card_width * self.scaler
            && mouse_pos.y >= self.position.expect(msg).y
            && mouse_pos.y <= self.position.expect(msg).y + self.card_height * self.scaler
    }

    pub fn reset_position(&mut self) {
        self.position = Option::None;
    }
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Deck { cards: Vec::new() }
    }

    pub fn add(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn first(&mut self) -> &mut Card {
        self.cards.first_mut().unwrap()
    }

    pub fn shuffle(&mut self) {
        // create a random usize x
        srand((get_time() * 10000.0) as u64);
        let x = gen_range(1, 1000);
        for _ in 0..x {
            self.cards.shuffle();
        }
        for card in self.cards.iter_mut() {
            card.reset_position();
            card.is_face_up = false;
        }
    }

    pub async fn fill_deck(&mut self) {
        if self.cards.is_empty() {
            for m in MajorArcana::iterator() {
                self.add(Card::new(Arcana::Major, Symbol::Major(m), "Classic".to_string()).await)
            }
            for m in NumericalValue::iterator() {
                for s in MinorArcana::iterator() {
                    self.add(
                        Card::new(Arcana::Minor, Symbol::Minor(s, m), "Classic".to_string()).await,
                    );
                }
            }
        }
    }
}
