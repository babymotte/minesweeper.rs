extern crate minesweeper;
extern crate rand;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{MinesweeperInterface, GameHandle, UiUpdate, GameState};
use std::sync::{Arc, Mutex};

fn main() {

    let interface = CliInterface {
        handle: Option::None,
        end: false,
    };
    let interface_arc = Arc::new(Mutex::new(interface));

    let mut handle = minesweeper::interface::start_game(interface_arc.clone(), Difficulty::Beginner);
    let width = handle.get_width();
    let height = handle.get_height();

    println!("");
    print_board(&handle);
    println!("");

    for i in 0..width * height {
        let x = i % width;
        let y = i / width;
        println!("Uncovering ({}, {})", x, y);
        handle.uncover(x, y);
    }

    println!("Done. Exiting...");
}

struct CliInterface {
    handle: Option<GameHandle>,
    end: bool,
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
            UiUpdate::TileUpdate(_, _, _) => {
                if let Option::Some(ref handle) = self.handle {
                    println!("");
                    print_board(handle);
                    println!("");
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