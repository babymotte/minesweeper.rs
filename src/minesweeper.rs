pub mod core;
pub mod stopwatch;
pub mod interface;

extern crate rand;
extern crate regex;
extern crate time;
extern crate timer;

#[cfg(test)]
mod core_test;
#[cfg(test)]
mod interface_test;
#[cfg(test)]
mod stopwatch_test;