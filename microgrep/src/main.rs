use colored::*;
use std::process;
use std::time::Instant;

use microgrep::run;

fn main() {
    let execution_start_time = Instant::now();

    let result = run();

    let elapsed = execution_start_time.elapsed();

    match result {
        Ok(_) => {
            let time_taken_msg = format!("\nTook {}ms to search.\n", elapsed.as_millis());
            println!("{}", time_taken_msg.yellow());
        }
        Err(reason) => {
            eprintln!("{}", reason);
            process::exit(1);
        }
    }
}
