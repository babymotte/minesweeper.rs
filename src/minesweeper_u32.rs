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
pub extern fn command(cmd: *const c_uint, callback: extern "C" fn(*const c_uint)) {
    
    let cmd = cmd as u32;

    /*
     * action   enum       mines      width/x    height/y
     *   00    | 00 | 0000 00000000 | 00000000 | 00000000
     */

     let action = cmd >> 30;

}

fn start(level: Difficulty, callback: extern "C" fn(*const c_uint)) {}
fn uncover_tile(x: usize, y: usize, callback: extern "C" fn(*const c_uint)) {}
fn toggle_flag(x: usize, y: usize, callback: extern "C" fn(*const c_uint)) {}

fn report_game_state_change(state: GameState, callback: extern "C" fn(*const c_uint)) {}
fn report_tile_state_change(state: TileState, callback: extern "C" fn(*const c_uint)) {}
fn report_time_change(state: Duration, callback: extern "C" fn(*const c_uint)) {}
fn report_new_highscore(state: Duration, callback: extern "C" fn(*const c_uint)) {}

fn convert_game_state_change(state: GameState) -> u32 {0}
fn convert_tile_state_change(state: TileState) -> u32 {0}
fn convert_time_change(state: Duration) -> u32 {0}
fn convert_new_highscore(state: Duration) -> u32 {0}

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

pub fn get_difficulty(cmd: u32) -> Difficulty {
    let level = (cmd & 0b_00_11_000000000000_00000000_00000000) >> 28 ;
    match level {
        0b_00 => Difficulty::Beginner,
        0b_01 => Difficulty::Intermediate,
        0b_10 => Difficulty::Expert,
        0b_11 => Difficulty::Custom(get_x(cmd), get_y(cmd), get_mines(cmd)),
        _ => panic!("This is impossible. All but two bits have bin masked out."),
    }
}