extern crate minesweeper;
extern crate rand;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{MinesweeperInterface, GameHandle, UiUpdate, GameState};
use std::sync::{Arc, Mutex};

fn main() {

    let interface = Arc::new(Mutex::new(CliInterface {end: false}));

    let mut handle = minesweeper::interface::start_game(interface.clone(), Difficulty::Beginner);

    println!("");
    print_board(&handle);
    println!("");

    let mut i = 0;
    let width = handle.get_width();

    while !interface.lock().unwrap().end {
        
        handle.uncover(i % width, i / width);

        println!("");
        print_board(&handle);
        println!("");

        i += 1;
    }
}

struct CliInterface {
    end: bool
}

impl MinesweeperInterface for CliInterface {
    fn update_ui(&mut self, update: UiUpdate) {
        match update {
            UiUpdate::GameStateUpdate(state) => {
                match state {
                    GameState::Won | GameState::Lost => self.end = true,
                    _ => {}
                }
            }
            _ => {}
        }
    }
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