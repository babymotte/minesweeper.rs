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

#[no_mangle]
pub extern fn command(name: *const c_uint) {}

#[no_mangle]
pub extern fn hook(callback: extern "C" fn(*const c_uint)) {}

fn start(level: Difficulty) {}
fn uncover_tile(x: usize, y: usize) {}
fn toggle_flag(x: usize, y: usize) {}

fn report_game_state_change(state: GameState) {}
fn report_tile_state_change(state: TileState) {}
fn report_time_change(state: Duration) {}
fn report_new_highscore(state: Duration) {}

fn convert_game_state_change(state: GameState) -> u32 {0}
fn convert_tile_state_change(state: TileState) -> u32 {0}
fn convert_time_change(state: Duration) -> u32 {0}
fn convert_new_highscore(state: Duration) -> u32 {0}