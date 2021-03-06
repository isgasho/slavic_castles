use crate::card::Card;
use crate::consts;
use crate::player::*;
use crate::ui::board_ui::BoardUI;
use crate::stats;
use quicksilver::{
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Event, Settings, State, Window},
    saving::{load, save},
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

#[derive(Serialize, Deserialize)]
pub struct Board {
    players: HashMap<PlayerNumer, Player>,
    active_player: PlayerNumer,
    pub time_before_next_move: f64,
    pub game_ended: bool,
    pub game_duration: f64,
    pub moves_made: i32,
    #[serde(skip)]
    pub ui: Option<BoardUI>,
}

impl Board {
    pub fn get() -> Self {
        if Board::has_save() {
            Board::load_board()
        } else {
            Board::new_board()
        }
    }

    pub fn has_save() -> bool {
        let result = load::<Self>("slavic_castles", "board");

        result.is_ok()
    }

    pub fn load_board() -> Self {
        let mut result = load::<Self>("slavic_castles", "board").expect("Could not load Board");
        if result.game_ended {
            result = Board::new_board();
        }
        result.ui = Board::create_ui();
        result.prepare_ui(true);
        result
    }

    pub fn save_board(&self) {
        save("slavic_castles", "board", &self).expect("Could not save Board");
    }

    pub fn new_board() -> Self {
        let mut players = HashMap::new();
        players.insert(PlayerNumer::First, Player::new(true, true));
        players.insert(PlayerNumer::Second, Player::new(false, false));

        let mut result = Board {
            players,
            active_player: PlayerNumer::First,
            time_before_next_move: 0.0,
            game_ended: false,
            game_duration: 0.0,
            moves_made: 0,
            ui: Board::create_ui(),
        };

        result.reset_game(false);

        result
    }

    fn create_ui() -> Option<BoardUI> {
        let ui = BoardUI::new().unwrap();
        Some(ui)
    }

    pub fn is_game_ended(&self) -> bool {
        self.game_ended
    }

    pub fn reset_game(&mut self, ai_only: bool) {
        self.players
            .get_mut(&PlayerNumer::First)
            .unwrap()
            .reset(true, !ai_only);
        self.players
            .get_mut(&PlayerNumer::Second)
            .unwrap()
            .reset(false, false);
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
        self.game_ended = false;
        self.game_duration = 0.0;
        self.moves_made = 0;
        self.prepare_ui(false);
        stats::Stats::game_started();
    }

    pub fn other_player(&self) -> PlayerNumer {
        if self.active_player == PlayerNumer::First {
            PlayerNumer::Second
        } else {
            PlayerNumer::First
        }
    }

    pub fn try_use_card(&mut self, card: &Card, index: i32, discard: bool) {
        let other_player_id = self.other_player();
        let mut player = self.players.get_mut(&self.active_player).unwrap();
        let ui = self.ui.as_mut().unwrap();
        if discard {
            player.replace_card(index);
            ui.send_message(format!("[{0}]Card discarded: {1}", self.active_player, card).as_str());
            self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;

            return;
        }

        if !card.can_aford(&player.resources) {
            return;
        }
        player.card_used(card, true);
        player.replace_card(index);

        let mut other_player = self.players.get_mut(&other_player_id).unwrap();
        other_player.card_used(card, false);

        ui.send_message(format!("[{0}]Card used: {1}", self.active_player, card).as_str());
        ui.card_used(card);
        ui.players_update(&self.players, self.active_player);
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
    }

    fn prepare_ui(&mut self, hide_help: bool) {
        let is_human_playing = self.is_human_playing();
        let ui = self.ui.as_mut().unwrap();
        ui.enable_ui_deck(is_human_playing);
        ui.update_deck(&self.players[&PlayerNumer::First]);
        ui.players_update(&self.players, self.active_player);
        ui.reset_game();
        if hide_help {
            ui.hide_help();
        }
    }

    fn can_active_player_move(&self) -> bool {
        self.time_before_next_move <= 0.0
    }

    fn is_human_playing(&self) -> bool {
        self.players[&self.active_player].is_human()
    }

    fn switch_player(&mut self) {
        self.moves_made += 1;
        self.active_player = self.other_player();
        self.players
            .get_mut(&self.active_player)
            .unwrap()
            .start_new_turn();
        let is_human_playing = self.is_human_playing();

        let ui = self.ui.as_mut().unwrap();
        ui.update_deck(&self.players[&PlayerNumer::First]);
        ui.enable_ui_deck(is_human_playing);
        ui.players_update(&self.players, self.active_player);
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
    }

    fn handle_move_end(&mut self) {
        if !self.players[&PlayerNumer::First].is_alive()
            || !self.players[&PlayerNumer::Second].is_alive()
        {
            let id = if self.players[&PlayerNumer::First].is_alive() {
                PlayerNumer::First
            } else {
                PlayerNumer::Second
            };
            self.ui
                .as_mut()
                .unwrap()
                .game_ended(self.players[&id].is_human());
            self.game_ended = true;
            stats::Stats::game_ended(self.players[&id].is_human(),self.game_duration,self.moves_made);

        } else if self.players[&PlayerNumer::First].has_max_possible_tower()
            || self.players[&PlayerNumer::Second].has_max_possible_tower()
        {
            let id = if self.players[&PlayerNumer::First].has_max_possible_tower() {
                PlayerNumer::First
            } else {
                PlayerNumer::Second
            };
            self.ui
                .as_mut()
                .unwrap()
                .game_ended(self.players[&id].is_human());
            self.game_ended = true;
            stats::Stats::game_ended(self.players[&id].is_human(),self.game_duration,self.moves_made);
        } else {
            self.switch_player();
        }
        self.save_board();
    }

    fn handle_mouse_input(&mut self, window: &mut Window) {
        let lmb_pressed = window.mouse()[MouseButton::Left] == ButtonState::Pressed;
        let rmb_pressed = window.mouse()[MouseButton::Right] == ButtonState::Pressed;
        let shift_pressed = window.keyboard()[Key::LShift] == ButtonState::Pressed;
        let mouse_pos = window.mouse().pos();
        if !lmb_pressed && !rmb_pressed {
            return;
        }

        if self.ui.as_mut().unwrap().end_game_hovered(mouse_pos) && lmb_pressed {
            self.reset_game(shift_pressed);
            return;
        }

        if !self.is_human_playing() || self.is_game_ended() || !self.can_active_player_move() {
            return;
        }

        self.ui.as_mut().unwrap().hide_help();

        let i = self
            .ui
            .as_mut()
            .unwrap()
            .card_index_on_pos(mouse_pos.x, mouse_pos.y);
        if i.is_some() {
            let card = self.players[&self.active_player].deck.cards[i.unwrap()];
            self.try_use_card(&card, i.unwrap() as i32, rmb_pressed || shift_pressed);
        }
    }

    fn handle_keyboard(&mut self, window: &mut Window) {
        self.ui.as_mut().unwrap().handle_keyboard(window);

        let shift_pressed = window.keyboard()[Key::LShift] == ButtonState::Pressed;

        if window.keyboard()[Key::R] == ButtonState::Pressed {
            self.reset_game(shift_pressed);
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.ui.as_mut().unwrap().update(delta);

        if self.is_game_ended() {
            return;
        }

        self.game_duration += delta;
        self.ui
            .as_mut()
            .unwrap()
            .update_game_info(self.game_duration, self.moves_made);
        if !self.can_active_player_move() {
            self.time_before_next_move -= delta;
            return;
        }

        if !self.players[&self.active_player].is_active() {
            self.handle_move_end();
        } else if !self.is_human_playing() {
            let (i, discard) = self.players[&self.active_player].get_possible_move();
            let card = self.players[&self.active_player].deck.cards[i as usize];
            self.try_use_card(&card, i, discard)
        }
    }

    pub fn event(&mut self, _event: &Event, window: &mut Window) -> Result<()> {
        match _event {
            Event::MouseMoved(_) => {
                let mouse_pos = window.mouse().pos();
                self.ui
                    .as_mut()
                    .unwrap()
                    .update_hovered_card(mouse_pos.x, mouse_pos.y);
            }
            Event::MouseButton(_, _) => self.handle_mouse_input(window),
            Event::Key(_, _) => self.handle_keyboard(window),
            _ => {}
        };

        Ok(())
    }
}
