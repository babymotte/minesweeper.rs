extern crate time;
extern crate timer;

use std::time::Duration;
use timer::{Timer, Guard};
use std::sync::mpsc::Sender;
use std::time::Instant;

pub struct Stopwatch {
    start: Option<Instant>,
    timer_guard: Option<Guard>
}

impl Stopwatch {

    pub fn new() -> Stopwatch {
        Stopwatch {
            start: Option::None,
            timer_guard: Option::None,
        }
    }

    pub fn start(&mut self, tx: Sender<Duration>) {

        println!("Starting Stopwatch.");

        let start = Instant::now();
        self.start = Option::Some(start.clone());

        let timer = Timer::new();
        let guard = timer.schedule_with_delay(time::Duration::seconds(1), move || {
            println!("Timer!");
            let delta = start.elapsed();
            tx.send(delta).unwrap();
        });

        self.timer_guard = Option::Some(guard);
    }

    pub fn stop(&mut self) -> Result<Duration, String> {

        println!("Stopping Stopwatch.");

        let result = match &self.timer_guard {
            &Some(_) => {
                let start = self.start.unwrap();
                let stop = Instant::now();
                let delta = stop.elapsed() - start.elapsed();
                Result::Ok(delta)
            },
            &None => Result::Err("Timer has not been started!".to_string()),
        };

        self.timer_guard = Option::None;

        result
    }
}