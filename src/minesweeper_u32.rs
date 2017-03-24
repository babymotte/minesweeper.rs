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

lazy_static! {
    static ref GAME_HANDLE: Arc<Mutex<Option<GameHandle>>> = Arc::new(Mutex::new(Option::None));
}

#[no_mangle]
pub extern fn command(name: *const c_uint, callback: extern "C" fn(*const c_uint)) {}

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