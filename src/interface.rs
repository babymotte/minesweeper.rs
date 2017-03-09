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
pub struct TileUpdate {
    x: usize,
    y: usize,
    state: TileState
}

pub struct GameHandle {
    level: Difficulty,
    board: Option<MineField>,
    tile_update_sender: Sender<Vec<TileUpdate>>,
    game_update_sender: Sender<GameState>,
}

impl TileUpdate {
    fn new(x: usize, y: usize, state: TileState) -> TileUpdate {
        TileUpdate{
            x: x,
            y: y,
            state: state
        }
    }
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

    pub fn give_up(&self) {
        let interface = &self.game_update_sender;
        let update = GameState::Lost;
        interface.send(update).unwrap();
    }

    pub fn get_tile_state(&self, x: usize, y: usize) -> TileState {
        match self.board {
            Option::None => TileState::Covered,
            Option::Some(ref board) => board.get_tile_state(x, y),
        }
    }

    fn do_unvocer(&mut self, x: usize, y: usize) -> TileState {
        let mut board = self.board.as_mut().unwrap();
        board.uncover(x, y)
    }

    fn uncover_impl(&mut self, x: usize, y: usize, changes: &mut Vec<TileUpdate>) {

        let result = self.do_unvocer(x, y);

        changes.push(TileUpdate::new(x, y, result));

        if let TileState::Uncovered(0) = result {
            self.uncover_nearby_mines(x, y, changes);
        }
    }

    pub fn uncover(&mut self, x: usize, y: usize) -> Vec<TileUpdate> {

        if let Option::None = self.board {
            let board: MineField = MineField::new(self.level, x, y);
            self.board = Option::Some(board);
        }

        let mut changes = Vec::new();
        self.uncover_impl(x, y, &mut changes);

        changes
        
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> TileUpdate {

        if let Option::None = self.board {
            let board: MineField = MineField::new(self.level, x, y);
            self.board = Option::Some(board);
        }

        let state = self.board.as_mut().unwrap().toggle_flag(x, y);
        TileUpdate::new(x, y, state)
    }

    fn uncover_nearby_mines(&mut self, x: usize, y: usize, changes: &mut Vec<TileUpdate>) {

        let mut uncover = Vec::<(usize, usize)>::new();

        if let Option::Some(ref board) = self.board {
            let neighbors = board.get_nearby_coordinates(x, y);
            let filtered = neighbors.iter().filter(|c| {
                let tile = board.get_tile(c.0, c.1);
                tile.get_state() == TileState::Covered
            });
            for c in filtered {
                uncover.push(*c);
            }
        }
        for c in uncover {
            self.uncover_impl(c.0, c.1, changes);
        }
    }
}

pub fn start_game(tile_update_sender: Sender<Vec<TileUpdate>>, game_update_sender: Sender<GameState>, level: Difficulty) -> GameHandle {

    GameHandle {
        level: level,
        board: Option::None,
        tile_update_sender: tile_update_sender,
        game_update_sender: game_update_sender,
    }
}