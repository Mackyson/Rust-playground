use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
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
    let mut file = File::open(path);
    match file.as_mut() {
        Err(e) => {
            eprintln!("could not open the file: {}", e);
            process::exit(1);
        }
        Ok(file) => {
            let start = Instant::now();
            single_threaded_hash(&file);
            let duration_single = start.elapsed();
            println!("single: {:?}", duration_single);
            
            if let Err(e) = file.seek(SeekFrom::Start(0)) {
                panic!("{}", e)
            };

            let start = Instant::now();
            multi_threaded_hash_std(&file);
            let duration_multi_std = start.elapsed();
            println!("multi-std: {:?}", duration_multi_std);

            if let Err(e) = file.seek(SeekFrom::Start(0)) {
                panic!("{}", e)
            };

            let start = Instant::now();
            multi_threaded_hash_parking_lot(&file);
            let duration_multi_parking_lot = start.elapsed();
            println!("multi-parking-lot: {:?}", duration_multi_parking_lot);

        }
    }
}

fn multi_threaded_hash_std(file: &File) {
    let results = Arc::new(Mutex::new(Vec::<(u32, Digest)>::new()));
    let mut threads = vec![];
    let lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let lines = Arc::new(Vec::<String>::from(lines));
    unsafe {
        if THREAD_NUM > lines.len() as u8 {
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
                let digest = (count, sha256::digest(lines.deref()[count as usize].clone()));
                results
                    .lock()
                    .unwrap()
                    .push(digest);
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    //results.lock().unwrap().sort();
    // output phase
    // for (num_row, digest) in results.lock().unwrap().iter() {
    //     println!("{}: {}", num_row, digest);
    // }
}

fn multi_threaded_hash_parking_lot(file: &File) {
    let results = Arc::new(parking_lot::Mutex::new(Vec::<(u32, Digest)>::new()));
    let mut threads = vec![];
    let lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let lines = Arc::new(Vec::<String>::from(lines));
    unsafe {
        if THREAD_NUM > lines.len() as u8 {
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
                let digest = (count, sha256::digest(lines.deref()[count as usize].clone()));
                results
                    .lock()
                    .push(digest);
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    //results.lock().unwrap().sort();
    // output phase
    // for (num_row, digest) in results.lock().iter() {
    //     println!("{}: {}", num_row, digest);
    // }
}

fn single_threaded_hash(file: &File) {
    let mut results = Vec::<(u32, Digest)>::new();
    let lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut counter: u32 = 0;
    for line in lines {
        results.push((counter, sha256::digest(line)));
        counter += 1;
    }

    // output phase
    //for (num_row, digest) in results.iter() {
    //    println!("{}: {}", num_row, digest);
    //}
}
