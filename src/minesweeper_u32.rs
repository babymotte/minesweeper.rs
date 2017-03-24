extern crate rand;
extern crate regex;

use std::io;
use std::time::Duration;
use core::{Difficulty, TileState};
use interface::{GameHandle, GameState};
use highscores::Highscores;
use highscores;
use regex::Regex;

pub fn start(level: Difficulty) {}
pub fn uncover_tile(x: usize, y: usize) {}
pub fn toggle_flag(x: usize, y: usize) {}

fn report_game_state_change(state: GameState) {}
fn report_tile_state_change(state: TileState) {}
fn report_time_change(state: Duration) {}
fn report_new_highscore(state: Duration) {}