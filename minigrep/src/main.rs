use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: minigrep <pattern> <path_to_file>");
        process::exit(1);
    }
    let (re, path) = parse_args(&args);
    let file = get_file(&path);
    if let Err(e) = file {
        println!("could not open the file: {}", e);
        process::exit(1);
    } else {
        let err = run(re, file.unwrap());
        if let Err(e) = err {
            panic!("{e}");
        } else {
            println!("done");
        }
    }
}

fn parse_args(args: &Vec<String>) -> (Regex, &Path) {
    let re = Regex::new(&args[1]).unwrap();
    let path = Path::new(&args[2]);

    (re, path)
}
fn get_file(path: &Path) -> Result<File, Box<dyn Error>> {
    let file = File::open(&path)?;
    Ok(file)
}
fn run(re: Regex, file: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if re.is_match(&line) {
                    println!("{line}");
                    continue;
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}
