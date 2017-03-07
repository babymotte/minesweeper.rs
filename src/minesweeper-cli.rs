extern crate minesweeper;
extern crate rand;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{GameHandle, UiUpdate, GameState};
use std::sync::mpsc;

fn main() {

    let (tx, rx) = mpsc::channel();

    let mut handle = minesweeper::interface::start_game(tx.clone(), Difficulty::Beginner);
    let width = handle.get_width();
    let height = handle.get_height();

    println!("");
    print_board(&handle);
    println!("");

    let mut lost = false;

    for i in 0..width * height {

        if lost {
            break;
        }

        let x = i % width;
        let y = i / width;
        println!("Uncovering ({}, {})", x, y);
        handle.uncover(x, y);

        let update = rx.recv().unwrap();
        lost = eval_update(update, &handle);
    }

    if lost {
        println!("You are dead!")
    } else {
        println!("Congratulations! You won!")
    }
}

fn eval_update(update: UiUpdate, handle: &GameHandle) -> bool {

    println!("");
    print_board(handle);
    println!("");
    match update {
        UiUpdate::GameStateUpdate(state) => state == GameState::Lost,
        _ => false,
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