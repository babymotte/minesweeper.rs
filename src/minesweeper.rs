pub mod core;
pub mod stopwatch;
pub mod interface;
pub mod highscores;
pub mod minesweeper_u32;
pub mod cli_io;

extern crate rand;
extern crate regex;
extern crate time;
extern crate timer;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate libc;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod core_test;
#[cfg(test)]
mod interface_test;
#[cfg(test)]
mod stopwatch_test;
#[cfg(test)]
mod highscores_test;
#[cfg(test)]
mod minesweeper_u32_test;