extern crate minesweeper;
extern crate rand;
extern crate regex;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{GameHandle, UiUpdate, GameState};
use std::sync::mpsc;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};
use regex::Regex;

enum Command {
    Uncover,
    Flag,
    Tile(usize, usize),
}

fn main() {

    let (tx, rx) = mpsc::channel();

    let handle = minesweeper::interface::start_game(tx.clone(), Difficulty::Beginner);

    println!("");
    print_board(&handle);
    println!("");

    let lost = Arc::new(Mutex::new(false));
    let handle = Arc::new(Mutex::new(handle));
    let cmd = Arc::new(Mutex::new(Command::Uncover));

    start_input_thread(handle.clone(), cmd.clone(), lost.clone());

    while !*lost.lock().unwrap() {
        println!("Eval loop");
        prompt_input(&cmd);
        let update = rx.recv().unwrap();
        println!("Received event!");
        *lost.lock().unwrap() = eval_update(update, &handle.lock().unwrap());
    }

    if *lost.lock().unwrap() {
        println!("You are dead!")
    } else {
        println!("Congratulations! You won!")
    }
}

fn prompt_input(cmd: &Arc<Mutex<Command>>) {
    let cmd = cmd.lock().unwrap();
    match *cmd {
           Command::Uncover => println!("Please enter the field you want to uncover in the form 'x,y':"),
           Command::Flag => println!("Please enter the field you want to mark with a flag in the form 'x,y':"),
           _ => panic!("Illegal state!")
    }
}

fn start_input_thread(handle: Arc<Mutex<GameHandle>>, cmd: Arc<Mutex<Command>>, lost: Arc<Mutex<bool>>) {
    thread::spawn(move || interactive_loop(handle, cmd, lost));
}

fn interactive_loop(handle: Arc<Mutex<GameHandle>>, cmd: Arc<Mutex<Command>>, lost: Arc<Mutex<bool>>) {

    let tile_coordinates_regex: Regex = Regex::new(r"^([0-9]+),([0-9]+)$").unwrap();
    
    while !*lost.lock().unwrap() {

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let new_cmd = parse_command(input.trim(), &tile_coordinates_regex);

        match new_cmd {
            Result::Ok(new_cmd) => {
                let mut cmd = cmd.lock().unwrap();
                match new_cmd {
                    Command::Tile(x,y) => match *cmd {
                        Command::Uncover => handle.lock().unwrap().uncover(x, y),
                        Command::Flag => handle.lock().unwrap().toggle_flag(x, y),
                        _ => panic!("Illegal state!")
                    },
                    _ => *cmd = new_cmd
                }
            },
            Result::Err(msg) => panic!(msg)
        }
    }
}

fn parse_command(cmd: &str, tile_coordinates_regex: &Regex) -> Result<Command, String> {
    match cmd {
        "u" | "uncover" => Result::Ok(Command::Uncover),
        "f" | "flag" => Result::Ok(Command::Flag),
        _ => match tile_coordinates_regex.captures(cmd) {
            Option::Some(caps) => {
                let x: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let y: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                Result::Ok(Command::Tile(x, y))
            },
            _ => Result::Err("Invalid command!".to_string())
        } 

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
        TileState::Marked => print!(" ✓"),
        _ => {}
    }
}