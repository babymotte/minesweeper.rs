use stopwatch::Stopwatch;
use std::sync::mpsc;


#[test]
fn simple_stopwatch_test() {

    let mut stopwatch = Stopwatch::new();

    let (tx, rx) = mpsc::channel();

    stopwatch.start(tx);

    for _ in 0..3 {
        match rx.recv() {
            Ok(time) => println!("{:?}", time),
            Err(msg) => panic!(msg),
        }
    }

    let result = stopwatch.stop();

    match result {
        Ok(duration) => assert_eq!(duration.as_secs(), 3),
        Err(msg) => panic!(msg),
    }
}