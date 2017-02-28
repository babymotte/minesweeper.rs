extern crate rand;
extern crate time;

use core::{Tile, TileState, Difficulty, MineField};
use time::Tm;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GameState {
	NotStarted,
	Started,
	Won,
	Lost
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum UiUpdate<'a> {
	TileUpdate(&'a Tile, TileState),
	TimeUpdate(&'a Tm),
	GameStateUpdate(GameState),
}

pub trait MinesweeperInterface {
	
	fn update_ui(&self, update: UiUpdate);
}

pub struct GameHandle<'a> {

	level: Difficulty,
	board: Option<&'a MineField>,
	interface: &'a MinesweeperInterface
}

impl<'a> GameHandle<'a> {
	
	pub fn uncover<>(&'a mut self, x: usize, y: usize) {
	
		if let Option::None = self.board {
			let board: MineField = MineField::new(self.level, x, y);
			let board: &'a MineField = &board;
			self.board = Option::Some(board);
		}
		
		let mut board: Option<&'a MineField> = self.board;
		let mut board: &'a MineField = board.unwrap();
		let mut tile: &mut Tile = board.get_tile(x, y);
		self.uncover_tile(tile)
	}
	
	pub fn toggle_flag(&mut self, x: usize, y: usize) {
	
		let mut board = &self.board;
		if let &Option::Some(ref board) = board {
			let mut tile = board.get_tile(x, y);
			self.toggle_flag_tile(tile)
		}
	
	}
	
	pub fn get_board(&self) -> &Option<&MineField> {
		&self.board
	}
	
	fn uncover_tile(&mut self, tile: &mut Tile) {
		
		let ref mut board = self.board;
		let result = match board {
			&mut Option::Some(ref mut board) => board.uncover(tile),
			_ => TileState::NoOp
		};
		
		match result {
			TileState::Uncovered(_) => self.interface.update_ui(UiUpdate::TileUpdate(tile, result)),
			TileState::Detonated => self.interface.update_ui(UiUpdate::TileUpdate(tile, result)),
			_ => {}
		};
	}
	
	fn toggle_flag_tile(&mut self, tile: &mut Tile) {
	
		let ref mut board = self.board;
		let result = match board {
			&mut Option::Some(ref mut board) => board.toggle_flag(tile),
			_ => TileState::NoOp
		};
		
		match result {
			_ => {}
		};
	}
}

pub fn start_game<I: MinesweeperInterface>(interface: &I, level: Difficulty) -> GameHandle {

	GameHandle {
		level: level,
		board: Option::None,
		interface: interface
	}
}