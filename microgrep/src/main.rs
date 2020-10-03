use std::process;
use std::time::Instant;

use microgrep::run;

fn main() {
    let execution_start_time = Instant::now();

    let result = run();

    let elapsed = execution_start_time.elapsed();

    match result {
        Ok(_) => println!("\nTook {}ms to search.", elapsed.as_millis()),
        Err(reason) => {
            eprintln!("{}", reason);
            process::exit(1);
        }
    }
}
