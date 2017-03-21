extern crate time;
extern crate timer;

use std::time::Duration;
use timer::{Timer, Guard};
use std::sync::mpsc::Sender;
use std::time::Instant;

pub struct Stopwatch {
    tx: Sender<Duration>,
    start: Option<Instant>,
    timer: Option<Timer>,
    timer_guard: Option<Guard>,
}

impl Stopwatch {
    pub fn new(tx: Sender<Duration>) -> Stopwatch {
        Stopwatch {
            tx: tx,
            start: Option::None,
            timer: Option::None,
            timer_guard: Option::None,
        }
    }

    pub fn start(&mut self) {

        let start = Instant::now();
        self.start = Option::Some(start.clone());

        let tx = self.tx.clone();

        let timer = Timer::new();
        let guard = timer.schedule_repeating(time::Duration::seconds(1), move || {
            let delta = start.elapsed();
            tx.send(delta).unwrap();
        });

        self.timer = Option::Some(timer);
        self.timer_guard = Option::Some(guard);
    }

    pub fn stop(&mut self) -> Result<Duration, String> {

        let result = match &self.timer_guard {
            &Some(_) => {
                let delta = self.start.unwrap().elapsed();
                Result::Ok(delta)
            }
            &None => Result::Err("Timer has not been started!".to_string()),
        };

        self.timer = Option::None;
        self.timer_guard = Option::None;

        result
    }
}