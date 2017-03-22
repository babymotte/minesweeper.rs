pub mod core;
pub mod stopwatch;
pub mod interface;
pub mod highscores;

extern crate rand;
extern crate regex;
extern crate time;
extern crate timer;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(test)]
mod core_test;
#[cfg(test)]
mod interface_test;
#[cfg(test)]
mod stopwatch_test;
#[cfg(test)]
mod highscores_test;