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
    StartGame = 0b00,
    UncoverTile = 0b01,
    ToggleFlag = 0b10,
    NotSpecified = 0b11,
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