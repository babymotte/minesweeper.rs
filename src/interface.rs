extern crate rand;
extern crate time;

use core;
use core::{TileState, Difficulty, MineField};
use time::Tm;
use std::sync::mpsc::Sender;


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

pub struct GameHandle {
    level: Difficulty,
    board: Option<MineField>,
    interface: Sender<UiUpdate>,
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

        let result = {
            let mut board = self.board.as_mut().unwrap();
            board.uncover(x, y)
        };

        match result {
            TileState::Uncovered(mine_count) => {
                {
                    let interface = &self.interface;
                    let update = UiUpdate::TileUpdate(x, y, result);
                    interface.send(update).unwrap();
                }
                if mine_count == 0 {
                    self.uncover_nearby_mines(x, y);
                }
            }
            TileState::Detonated => {
                let interface = &self.interface;
                let update = UiUpdate::GameStateUpdate(GameState::Lost);
                interface.send(update).unwrap();
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

    fn uncover_nearby_mines(&mut self, x: usize, y: usize) {

        let mut uncover = Vec::<(usize, usize)>::new();

        if let Option::Some(ref board) = self.board {
            let neighbors = board.get_nearby_coordinates(x, y);
            let filtered = neighbors.iter().filter(|c| {
                let tile = board.get_tile(c.0, c.1);
                tile.get_state() == TileState::Covered && tile.get_nearby_mines() == 0
            });
            for c in filtered {
                uncover.push(*c);
            }
        }
        for c in uncover {
            self.uncover(c.0, c.1);
        }
    }
}

pub fn start_game(interface: Sender<UiUpdate>, level: Difficulty) -> GameHandle {

    GameHandle {
        level: level,
        board: Option::None,
        interface: interface,
    }
}