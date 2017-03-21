use stopwatch::Stopwatch;
use std::sync::mpsc;


#[test]
fn simple_stopwatch_test() {

    let (tx, rx) = mpsc::channel();

    let mut stopwatch = Stopwatch::new(tx);

    stopwatch.start();

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