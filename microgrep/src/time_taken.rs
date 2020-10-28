use colored::*;
use std::time;

pub fn time_taken_to_search(beginning_timestamp: time::Instant) -> String {
    let elapsed_time = beginning_timestamp.elapsed();

    let time_taken_msg = format!("\nTook {}ms to search.\n", elapsed_time.as_millis());

    String::from(format!("{}", time_taken_msg.yellow()))
}
