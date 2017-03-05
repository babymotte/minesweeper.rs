extern crate rand;
extern crate time;

use core;
use core::{TileState, Difficulty, MineField};
use time::Tm;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GameState {
    NotStarted,
    Started,
    Won,
    Lost,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum UiUpdate {
    TileUpdate(usize, usize, TileState),
    TimeUpdate(Tm),
    GameStateUpdate(GameState),
}

pub trait MinesweeperInterface {
    fn update_ui(&mut self, update: UiUpdate);
}

pub struct GameHandle {
    level: Difficulty,
    board: Option<MineField>,
    interface: Arc<Mutex<MinesweeperInterface>>
}

impl GameHandle {
    pub fn get_width(&self) -> usize {
        core::get_params_for_difficulty(self.level).0
    }

    pub fn get_height(&self) -> usize {
        core::get_params_for_difficulty(self.level).1
    }

    pub fn get_mines(&self) -> usize {
        core::get_params_for_difficulty(self.level).2
    }

    pub fn get_board(&self) -> &Option<MineField> {
        &self.board
    }

    pub fn get_tile_state(&self, x: usize, y: usize) -> TileState {
        match self.board {
            Option::None => TileState::Covered,
            Option::Some(ref board) => board.get_tile_state(x, y),
        }
    }

    pub fn uncover(&mut self, x: usize, y: usize) {

        if let Option::None = self.board {
            let board: MineField = MineField::new(self.level, x, y);
            self.board = Option::Some(board);
        }

        let mut board = self.board.as_mut().unwrap();

        let result = board.uncover(x, y);

        match result {
            TileState::Uncovered(_) |
            TileState::Detonated => {
                let interface = &self.interface;
                let interface = interface.lock();
                let mut interface = interface.unwrap();
                interface.update_ui(UiUpdate::TileUpdate(x, y, result));
            }
            _ => {}
        };
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {

        if let Option::None = self.board {
            return;
        }

        let mut board = self.board.as_mut().unwrap();

        board.toggle_flag(x, y);
    }
}

pub fn start_game(interface: Arc<Mutex<MinesweeperInterface>>, level: Difficulty) -> GameHandle {

    GameHandle {
        level: level,
        board: Option::None,
        interface: interface,
    }
}