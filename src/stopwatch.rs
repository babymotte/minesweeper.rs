extern crate time;
extern crate timer;

use std::time::Duration;
use timer::{Timer, Guard};
use std::sync::mpsc::Sender;
use std::time::Instant;

pub struct Stopwatch {
    tx: Option<Sender<Duration>>,
    start: Option<Instant>,
    timer: Option<Timer>,
    timer_guard: Option<Guard>,
}

impl Stopwatch {
    pub fn new(tx: Option<Sender<Duration>>) -> Stopwatch {
        Stopwatch {
            tx: tx,
            start: Option::None,
            timer: Option::None,
            timer_guard: Option::None,
        }
    }

    pub fn start(&mut self) {

        if let Option::Some(_) = self.start {
            panic!("Stopwatch has already been started!");
        }

        let start = Instant::now();
        self.start = Option::Some(start.clone());

        if let Option::Some(tx) = self.tx.clone() {

            let timer = Timer::new();
            let guard = timer.schedule_repeating(time::Duration::seconds(1), move || {
                let delta = start.elapsed();
                tx.send(delta).unwrap();
            });

            self.timer = Option::Some(timer);
            self.timer_guard = Option::Some(guard);
        }
    }

    pub fn stop(&mut self) -> Result<Duration, String> {

        let result = match self.start {
            Some(start) => {
                let delta = start.elapsed();
                Result::Ok(delta)
            }
            None => Result::Err("Stopwatch has not been started!".to_string()),
        };

        // make sure timer gets dropped
        self.timer = Option::None;
        self.timer_guard = Option::None;

        result
    }
}