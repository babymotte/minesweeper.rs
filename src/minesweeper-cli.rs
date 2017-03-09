extern crate minesweeper;
extern crate rand;
extern crate regex;

use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{GameHandle, GameState};
use std::cell::Cell;
use std::io;
use regex::Regex;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Command {
    Uncover,
    Flag,
    Tile(usize, usize),
}

fn main() {

    let game_state = Cell::new(GameState::NotStarted);
    let level = Difficulty::Beginner;
    let handle = create_game_handle(game_state.clone(), level);

    print_board(&handle);

    start_input_loop(handle, game_state.clone());

    bye(game_state.get());
}

fn finished(game_state: &Cell<GameState>) -> bool {
    match game_state.get() {
        GameState::Won | GameState::Lost => true,
        _ => false,
    }
}

fn bye(state: GameState) {
    match state {
        GameState::Won => println!("Congratulations! You won!"),
        GameState::Lost => println!("You are dead!"),
        _ => println!("You're neither dead nor have you won, yet somehow this game is over. Weird. ({:?})", state),
    }
}

fn create_game_handle(game_state: Cell<GameState>, level: Difficulty) -> GameHandle {
    minesweeper::interface::start_game(game_state, level)
}

fn start_input_loop(mut handle: GameHandle, game_state: Cell<GameState>) {

    let tile_coordinates_regex: Regex = Regex::new(r"^([0-9]+),([0-9]+)$").unwrap();

    let mut cmd = Command::Uncover;
    
    while !finished(&game_state) {

        println!("Please enter a command or \"help\" to print a list of all available commands:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let new_cmd = parse_command(input.trim(), &tile_coordinates_regex);

        match new_cmd {
            Result::Ok(new_cmd) => {
                match new_cmd {
                    Command::Tile(x,y) => match cmd {
                        Command::Uncover => {
                            handle.uncover(x, y);
                            print_board(&handle);
                        },
                        Command::Flag => {
                            handle.toggle_flag(x, y);
                            print_board(&handle);
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