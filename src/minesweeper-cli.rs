extern crate minesweeper;
extern crate rand;

use minesweeper::core::{MineField, Difficulty, TileState};
use minesweeper::interface::{MinesweeperInterface, GameHandle};

fn main() {

	let handle = minesweeper::interface::start_game();
	
	println!("");
	print_board(&mut board);
	println!("");
}

struct CliInterface {

}

impl MinesweeperInterface for CliInterface {
	
	fn update_ui(&self, update: UiUpdate) {
		
	}
}



fn print_board(board: &mut MineField) {

	for _ in 0 .. board.get_width() {
		print!("----");
	}
	println!("-");

	for y in 0 .. board.get_height() {
		for x in 0 .. board.get_width() {
			print(board.uncover(x,y));
		}
		print!("|");
		println!("");
		for _ in 0 .. board.get_width() {
			print!("----");
		}
		println!("-");
	}
}

fn print(state: TileState) {
	match state {
		TileState::Uncovered(0) => print!("|   "),
		TileState::Uncovered(x) => print!("| {} ", x),
		TileState::Detonated =>  print!("| * "),
		_ => {}
	}
}