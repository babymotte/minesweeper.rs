extern crate rand;
extern crate regex;
extern crate libc;

use std::io;
use std::time::Duration;
use core::{Difficulty, TileState};
use interface::{GameHandle, GameState};
use highscores::Highscores;
use highscores;
use regex::Regex;
use libc::c_uint;
use std::sync::{Arc, Mutex};
use std::mem;


/*  
 * Protocol
 * 
 * Command coming from calling programm:

 * action   enum      mines      width/x    height/y
 *   00    | 00 | 000000000000 | 00000000 | 00000000
 *
 *
 * Return value to calling program:
 *
 * type enum      time           x     y/neighbors
 *  00 | 00 | 000000000000 | 00000000 | 0000|0000
 */

lazy_static! {
    static ref GAME_HANDLE: Arc<Mutex<Option<GameHandle>>> = Arc::new(Mutex::new(Option::None));
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    StartGame = 0b_00,
    UncoverTile = 0b_01,
    ToggleFlag = 0b_10,
    NotSpecified = 0b_11,
}

#[no_mangle]
pub extern "C" fn command(cmd: *const c_uint, callback: extern "C" fn(*const c_uint)) {

    let cmd = cmd as u32;

    let action = get_action(cmd);
    match action {
        Action::StartGame => {
            let level = get_difficulty(cmd);
            start(level, callback);
        }
        Action::UncoverTile => {
            let (x, y) = get_x_y(cmd);
            uncover_tile(x, y, callback);
        }
        Action::ToggleFlag => {
            let (x, y) = get_x_y(cmd);
            toggle_flag(x, y, callback);
        }
        Action::NotSpecified => {}
    }

}

fn start(level: Difficulty, callback: extern "C" fn(*const c_uint)) {}
fn uncover_tile(x: usize, y: usize, callback: extern "C" fn(*const c_uint)) {}
fn toggle_flag(x: usize, y: usize, callback: extern "C" fn(*const c_uint)) {}

fn report_game_state_change(state: GameState, callback: extern "C" fn(*const c_uint)) {}
fn report_tile_state_change(state: TileState, callback: extern "C" fn(*const c_uint)) {}
fn report_time_change(state: Duration, callback: extern "C" fn(*const c_uint)) {}
fn report_new_highscore(state: Duration, callback: extern "C" fn(*const c_uint)) {}

pub fn convert_game_state_change(state: GameState) -> u32 {
    (state as u32) << 28
}

pub fn convert_tile_state_change(state: TileState) -> u32 {
    match state {
        TileState::Covered => 0b_01_00_000000000000_00000000_00000000,
        TileState::Marked => 0b_01_01_000000000000_00000000_00000000,
        TileState::Uncovered(neighbors) => {
            (0b_01_10_000000000000_00000000_00000000 | (neighbors as u32))
        }
        TileState::Detonated => 0b_01_11_000000000000_00000000_00000000,
        _ => panic!("NoOp not allowed!"),
    }
}

pub fn convert_time_change(duration: Duration) -> u32 {
    let seconds = (duration.as_secs() * 1_000) as u32;
    let nanos = (duration.subsec_nanos() / 1_000_000) as u32; 
    let millis = seconds + nanos;
    0b_10_000000_00000000_00000000_00000000 | millis
}

pub fn convert_new_highscore(state: Duration) -> u32 {
    0b_11_000000_00000000_00000000_00000000
}

pub fn get_action(cmd: u32) -> Action {
    unsafe { mem::transmute((cmd >> 30) as u8) }
}

pub fn get_x(cmd: u32) -> usize {
    ((cmd & 0b_00_00_000000000000_11111111_00000000) >> 8) as usize
}

pub fn get_y(cmd: u32) -> usize {
    (cmd & 0b_00_00_000000000000_00000000_11111111) as usize
}

pub fn get_mines(cmd: u32) -> usize {
    ((cmd & 0b_00_00_111111111111_00000000_00000000) >> 16) as usize
}

pub fn get_x_y(cmd: u32) -> (usize, usize) {
    (get_x(cmd), get_y(cmd))
}

pub fn get_difficulty(cmd: u32) -> Difficulty {
    let level = (cmd & 0b_00_11_000000000000_00000000_00000000) >> 28;
    match level {
        0b_00 => Difficulty::Beginner,
        0b_01 => Difficulty::Intermediate,
        0b_10 => Difficulty::Expert,
        0b_11 => Difficulty::Custom(get_x(cmd), get_y(cmd), get_mines(cmd)),
        _ => panic!("This is impossible. All but two bits have bin masked out."),
    }
}