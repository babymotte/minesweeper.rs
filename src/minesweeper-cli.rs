extern crate minesweeper;
extern crate rand;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{GameHandle, UiUpdate, GameState};
use std::sync::mpsc;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {

    let (tx, rx) = mpsc::channel();

    let handle = minesweeper::interface::start_game(tx.clone(), Difficulty::Beginner);

    println!("");
    print_board(&handle);
    println!("");

    let mut lost = false;

    let handle = Arc::new(Mutex::new(handle));

    {
        let handle = handle.clone();
        thread::spawn(move || while !lost {

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let split = input.split(",").collect::<Vec<&str>>();
            let x: usize = split[0].trim().parse().expect("Please type a number!");
            let y: usize = split[1].trim().parse().expect("Please type a number!");

            handle.lock().unwrap().uncover(x, y);
        });
    }

    while !lost {
        println!("Please enter the field you want to uncover in the form 'x,y':");
        let update = rx.recv().unwrap();
        lost = eval_update(update, &handle.lock().unwrap());
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