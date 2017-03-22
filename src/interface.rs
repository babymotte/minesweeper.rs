extern crate rand;
extern crate time;

use std::sync::mpsc::Sender;
use std::time::Duration;
use core;
use core::{TileState, Difficulty, MineField};
use stopwatch::Stopwatch;


#[derive(Debug, Copy, Clone)]
pub enum GameState {
    NotStarted,
    Started,
    Won,
    Lost,
}

#[derive(Debug, Copy, Clone)]
pub struct TileUpdate {
    x: usize,
    y: usize,
    state: TileState,
}

pub struct GameHandle {
    stopwatch: Stopwatch,
    level: Difficulty,
    board: Option<MineField>,
    game_state: GameState,
}

impl TileUpdate {
    fn new(x: usize, y: usize, state: TileState) -> TileUpdate {
        TileUpdate {
            x: x,
            y: y,
            state: state,
        }
    }
}

impl GameHandle {
    pub fn new(level: Difficulty, stopwatch_tx: Sender<Duration>) -> GameHandle {
        GameHandle {
            stopwatch: Stopwatch::new(stopwatch_tx),
            level: level,
            board: Option::None,
            game_state: GameState::NotStarted,
        }
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

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

    pub fn give_up(&mut self) {
        let update = GameState::Lost;
        self.game_state = update;
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

        match result {
            TileState::Uncovered(0) => self.uncover_nearby_mines(x, y, changes),
            TileState::Detonated => self.game_state = GameState::Lost,
            _ => {}
        }

        if self.has_won() {
            self.game_state = GameState::Won;
        }
    }

    fn has_won(&self) -> bool {

        match self.game_state {
            GameState::Won => true,
            GameState::Lost | GameState::NotStarted => false,
            GameState::Started => {
                match self.board {
                    Option::Some(ref board) => board.is_clear(),
                    _ => false,
                }
            }
        }
    }

    fn init_board(&mut self, x: usize, y: usize) {
        let board: MineField = MineField::new(self.level, x, y);
        self.board = Option::Some(board);
        self.game_state = GameState::Started;
        self.stopwatch.start();
    }

    pub fn uncover(&mut self, x: usize, y: usize) -> Vec<TileUpdate> {

        if let Option::None = self.board {
            self.init_board(x, y);
        }

        let mut changes = Vec::new();
        self.uncover_impl(x, y, &mut changes);

        changes

    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> TileUpdate {

        if let Option::None = self.board {
            self.init_board(x, y);
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