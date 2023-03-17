use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicU32,Ordering};
use std::sync::{Arc, Mutex};
use std::{process, thread};
type Digest = String;

static mut THREAD_NUM: u8 = 8;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./multi-thread-hashing <path-to-file>");
        process::exit(1);
    }
    let path = Path::new(&args[1]);
    let file = File::open(path);
    match file {
        Err(e) => {
            eprintln!("could not open the file: {}", e);
            process::exit(1);
        }
        Ok(file) => {
            let results = Arc::new(Mutex::new(Vec::<(u32, Digest)>::new()));
            let mut threads = vec![];
            let lines: Vec<String> = BufReader::new(&file).lines().map(|x| x.unwrap()).collect();
            let lines = Arc::new(Vec::<String>::from(lines));
            unsafe {
                if THREAD_NUM < lines.len() as u8 {
                    THREAD_NUM = lines.len() as u8
                }
            };
            let counter = Arc::new(AtomicU32::new(0));
            for _ in unsafe { 0..THREAD_NUM } {
                let counter = counter.clone();
                let results = results.clone();
                let lines = lines.clone();
                let max_len = lines.len();
                let thread = thread::spawn(move || {
                    loop {
                        let count = counter.fetch_add(1, Ordering::Relaxed);
                        if count as usize >= max_len {
                            break;
                            // This index checking incurs more or less performance degradation by add some CPU operations in every loop.
                        }
                        results
                            .lock()
                            .unwrap()
                            .push((count, sha256::digest(lines.deref()[count as usize].clone())));
                    }
                });
                threads.push(thread);
            }
            for thread in threads {
                thread.join().unwrap();
            }
            results.lock().unwrap().sort();
            for (num_row, digest) in results.lock().unwrap().iter() {
                println!("{}: {}", num_row, digest);
            }
        }
    }
}
