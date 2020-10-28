use colored::*;
use std::time;

pub fn time_taken_to_search(beginning_timestamp: time::Instant) -> String {
    let elapsed_time = beginning_timestamp.elapsed();

    let time_taken_msg = format!("\nTook {}ms to search.\n", elapsed_time.as_millis());

    String::from(format!("{}", time_taken_msg.yellow()))
}

#[cfg(test)]
mod tests {

    use crate::time_taken_to_search;

    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn takes_less_than_one_ms() {
        let start = Instant::now();

        let returned_msg = time_taken_to_search(start);

        print!("{}", returned_msg);

        assert!(returned_msg.contains("Took 0ms to search."));
    }

    #[test]
    fn takes_one_ms() {
        let start = Instant::now();

        thread::sleep(Duration::from_millis(1));

        let returned_msg = time_taken_to_search(start);

        print!("{}", returned_msg);

        assert!(returned_msg.contains("Took 1ms to search."));
    }

    #[test]
    fn takes_more_than_one_ms() {
        let start = Instant::now();

        thread::sleep(Duration::from_millis(50));

        let returned_msg = time_taken_to_search(start);

        print!("{}", returned_msg);

        assert!(returned_msg.contains("Took 50ms to search."));
    }

    #[test]
    fn takes_more_than_one_sec() {
        let start = Instant::now();

        thread::sleep(Duration::from_secs(1));

        let returned_msg = time_taken_to_search(start);

        print!("{}", returned_msg);

        assert!(returned_msg.contains("Took 1000ms to search."));
    }
}
