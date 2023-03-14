use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process;
use tempfile::tempfile;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_search() -> Result<(), Box<dyn Error>> {
        let re1 = Regex::new(r"safe").unwrap();
        let re2 = Regex::new(r"\d").unwrap();
        let mut file = tempfile()?;
        writeln!(
            file,
            "\
Rust:
safe, fast, productive.
Pick 3."
        )?;
        file.seek(SeekFrom::Start(0))?;
        assert_eq!(
            file_search(&re1, &file).unwrap(),
            vec!["safe, fast, productive."]
        );
        file.seek(SeekFrom::Start(0))?;
        assert_eq!(file_search(&re2, &file).unwrap(), vec!["Pick 3."]);
        drop(file);
        Ok(())
    }
}
pub fn run(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() != 3 {
        println!("Usage: minigrep <pattern> <path_to_file>");
        process::exit(1);
    }
    let (re, path) = parse_args(&args);
    let file = get_file(&path)?;
    let lines = file_search(&re, &file)?;
    for line in lines {
        println!("{line}");
    }
    Ok(())
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

fn file_search(re: &Regex, file: &File) -> Result<Vec<String>, Box<dyn Error>> {
    let mut lines = Vec::<String>::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if re.is_match(&line) {
                    lines.push(line);
                    continue;
                }
            }
            Err(e) => {
                println!("err!");
                return Err(Box::new(e));
            }
        }
    }
    dbg!(&lines);
    Ok(lines)
}
