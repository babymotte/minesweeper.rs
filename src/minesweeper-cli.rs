extern crate minesweeper;
extern crate rand;
extern crate regex;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{GameHandle, TileUpdate, GameState};
use std::sync::mpsc;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use regex::Regex;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Command {
    Uncover,
    Flag,
    Tile(usize, usize),
}

fn main() {

    let level = Difficulty::Beginner;
    let (tile_tx, tile_rx) = mpsc::channel();
    let (game_tx, game_rx) = mpsc::channel();
    let handle = create_game_handle(tile_tx.clone(), game_tx.clone(), level);

    print_board(&handle);

    let game_state = Arc::new(Mutex::new(GameState::NotStarted));
    let handle = Arc::new(Mutex::new(handle));

    start_game_state_listener(game_rx, game_state.clone());
    start_input_loop(handle.clone(), game_state.clone());

    bye(*game_state.lock().unwrap());
}

fn finished(game_state: &Arc<Mutex<GameState>>) -> bool {
    match *game_state.lock().unwrap() {
        GameState::Won | GameState::Lost => true,
        _ => false,
    }
}

fn start_game_state_listener(game_rx: Receiver<GameState>, game_state: Arc<Mutex<GameState>>) {
    thread::spawn(move || game_state_loop(game_rx, game_state));
}

fn game_state_loop(game_rx: Receiver<GameState>, game_state: Arc<Mutex<GameState>>) {
     while !finished(&game_state) {
        let update = game_rx.recv().unwrap();
        *game_state.lock().unwrap() = update;
     }
}

fn bye(state: GameState) {
    match state {
        GameState::Won => println!("Congratulations! You won!"),
        GameState::Lost => println!("You are dead!"),
        _ => println!("You're neither dead nor have you won, yet somehow this game is over. Weird. ({:?})", state),
    }
}

fn create_game_handle(tile_update_sender: Sender<Vec<TileUpdate>>, game_update_sender: Sender<GameState>, level: Difficulty) -> GameHandle {
    minesweeper::interface::start_game(tile_update_sender, game_update_sender, level)
}

fn start_input_loop(handle: Arc<Mutex<GameHandle>>, game_state: Arc<Mutex<GameState>>) {

    let tile_coordinates_regex: Regex = Regex::new(r"^([0-9]+),([0-9]+)$").unwrap();

    let mut cmd = Command::Uncover;
    
    while !finished(&game_state) {

        println!("Please enter a command er \"help\" to print a list of all available commands:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let new_cmd = parse_command(input.trim(), &tile_coordinates_regex);

        match new_cmd {
            Result::Ok(new_cmd) => {
                match new_cmd {
                    Command::Tile(x,y) => match cmd {
                        Command::Uncover => {
                            handle.lock().unwrap().uncover(x, y);
                            print_board(&*handle.lock().unwrap());
                        },
                        Command::Flag => {
                            handle.lock().unwrap().toggle_flag(x, y);
                            print_board(&*handle.lock().unwrap());
                        },
                        _ => panic!("Illegal state!"),
                    },
                    _ => {
                        cmd = new_cmd;
                        println!("Switching to command mode {:?}", new_cmd);
                    }
                }
            },
            Result::Err(msg) => {
                println!("{}", msg);
            }
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
            _ => Result::Err("Unknown command: ".to_string() + cmd)
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
    println!("");
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