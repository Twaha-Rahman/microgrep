use std::process;

use microgrep::run;

fn main() {
    let result = run();

    match result {
        Ok(time_taken_msg) => {
            println!("{}", time_taken_msg);
        }
        Err(reason) => {
            eprintln!("{}", reason);
            process::exit(1);
        }
    }
}
