extern crate minesweeper;
extern crate rand;
extern crate regex;

use std::io;
use std::time::Duration;
use minesweeper::core::{Difficulty, TileState};
use minesweeper::interface::{GameHandle, GameState};
use minesweeper::highscores::Highscores;
use minesweeper::highscores;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Command {
    Uncover,
    Flag,
    Tile(usize, usize),
    NoOp,
}

fn main() {
    
    println!("Hello");
    
    let level = Difficulty::Beginner;
    let handle = GameHandle::new(level, Option::None);

    print_board(&handle);

    let final_state = run_input_loop(handle);

    bye(final_state.0, final_state.1, level);
}

fn finished(game_state: GameState) -> bool {
    match game_state {
        GameState::Won | GameState::Lost => true,
        _ => false,
    }
}

fn bye(state: GameState, duration: Duration, level: Difficulty) {
    match state {
        GameState::Won => {
            println!("Congratulations! You won!");
            check_highscores(level, duration);
        }
        GameState::Lost => println!("You are dead!"),
        _ => panic!("The game cannot be over if the player has neither won nor lost!"),
    }
    println!("Game duration: {} seconds", duration.as_secs());
}

fn check_highscores(level: Difficulty, duration: Duration) {

    let mut hs = highscores::load("highscores");
    let secs = duration.as_secs() as u64;
    let nanos = duration.subsec_nanos() as u64;
    let ms = secs * 1_000 + nanos / 1_000_000;

    match level {
        Difficulty::Beginner => update_beginner_time(&mut hs, ms),
        Difficulty::Intermediate => update_intermediate_time(&mut hs, ms),
        Difficulty::Expert => update_expert_time(&mut hs, ms),
        Difficulty::Custom(_, _, _) => {}
    }

    highscores::save(&hs, "highscores");
}

fn update_beginner_time(hs: &mut Highscores, duration: u64) {
    match hs.get_beginner() {
        Option::Some(beginner) => {
            if duration < beginner {
                print!("New highscore!");
                hs.set_beginner(duration);
            }
        }
        Option::None => {
            print!("New highscore!");
            hs.set_beginner(duration);
        }
    }
}

fn update_intermediate_time(hs: &mut Highscores, duration: u64) {
    match hs.get_intermediate() {
        Option::Some(beginner) => {
            if duration < beginner {
                print!("New highscore!");
                hs.set_intermediate(duration);
            }
        }
        Option::None => {
            print!("New highscore!");
            hs.set_intermediate(duration);
        }
    }
}
fn update_expert_time(hs: &mut Highscores, duration: u64) {
    match hs.get_expert() {
        Option::Some(beginner) => {
            if duration < beginner {
                print!("New highscore!");
                hs.set_expert(duration);
            }
        }
        Option::None => {
            print!("New highscore!");
            hs.set_expert(duration);
        }
    }
}

fn run_input_loop(mut handle: GameHandle) -> (GameState, Duration) {

    let tile_coordinates_regex: Regex = Regex::new(r"^([0-9]+),([0-9]+)$").unwrap();

    let mut cmd = Command::Uncover;

    while !finished(handle.get_game_state()) {

        println!("Please enter a command or \"help\" to print a list of all available commands:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let new_cmd = parse_command(input.trim(), &tile_coordinates_regex);

        match new_cmd {
            Result::Ok(new_cmd) => {
                match new_cmd {
                    Command::Tile(x, y) => {
                        match cmd {
                            Command::Uncover => {
                                handle.uncover(x, y);
                                print_board(&handle);
                            }
                            Command::Flag => {
                                handle.toggle_flag(x, y);
                                print_board(&handle);
                            }
                            _ => panic!("Illegal state!"),
                        }
                    }
                    Command::NoOp => {}
                    _ => {
                        cmd = new_cmd;
                        println!("Switching to command mode {:?}", new_cmd);
                    }
                }
            }
            Result::Err(msg) => {
                println!("{}", msg);
            }
        }
    }

    // unwrapping the duration because there is no legal way for it not to be set at this point
    (handle.get_game_state(), handle.get_duration().unwrap())
}

fn parse_command(cmd: &str, tile_coordinates_regex: &Regex) -> Result<Command, String> {
    match cmd {
        "u" | "uncover" => Result::Ok(Command::Uncover),
        "f" | "flag" => Result::Ok(Command::Flag),
        "h" | "help" => print_help(),
        _ => {
            match tile_coordinates_regex.captures(cmd) {
                Option::Some(caps) => {
                    let x: usize = caps.get(1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap();
                    let y: usize = caps.get(2)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap();
                    Result::Ok(Command::Tile(x, y))
                }
                _ => Result::Err("Unknown command: ".to_string() + cmd),
            }
        } 

    }
}

fn print_help() -> Result<Command, String> {

    println!("");
    println!("Available commands:");
    println!("");
    println!(" <x>,<y>\t\tPerform an action on the tile with the given x and \
              y\n\t\t\tcoordinates. The action depends on the current mode.\n");
    println!(" u | uncover\t\tChange to uncover mode.\n\t\t\tEntering coordinates will uncover \
              the mine at that\n\t\t\tposition (default in a new game).\n");
    println!(" f | flag\t\tChange to flag mode.\n\t\t\tEntering coordinates will mark the mine \
              at that\n\t\t\tposition with a flag.\n");
    println!(" h | help\t\tShow this message.\n");
    println!("");

    Result::Ok(Command::NoOp)
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