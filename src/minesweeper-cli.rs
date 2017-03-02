extern crate minesweeper;
extern crate rand;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{MinesweeperInterface, GameHandle, UiUpdate};
use std::sync::Arc;

fn main() {

    let interface = Arc::new(CliInterface {});

    let mut handle = minesweeper::interface::start_game(interface, Difficulty::Beginner);

    println!("");
    print_board(&handle);
    println!("");

    handle.uncover(0, 0);

    println!("");
    print_board(&handle);
    println!("");

    handle.uncover(1, 0);

    println!("");
    print_board(&handle);
    println!("");
    
    handle.uncover(2, 0);
}

struct CliInterface {}

impl MinesweeperInterface for CliInterface {
    fn update_ui(&self, update: UiUpdate) {}
}

fn print_board(handle: &GameHandle) {

    println!("");

    for y in 0..handle.get_height() {
        for x in 0..handle.get_width() {
            print(handle.get_tile_state(x, y));
        }
        println!("");
    }
}

fn print(state: TileState) {
    match state {
        TileState::Uncovered(0) => print!("  "),
        TileState::Uncovered(x) => print!(" {}", x),
        TileState::Covered => print!(" ■"),
        TileState::Detonated => print!(" *"),
        _ => {}
    }
}