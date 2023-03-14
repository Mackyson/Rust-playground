extern crate minigrep;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let err = minigrep::run(&args);
    if let Err(e) = err {
        eprintln!("{e}");
        process::exit(1);
    } else {
        println!("done");
    }
}
