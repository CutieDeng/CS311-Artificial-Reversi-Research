use std::{collections::{BTreeMap}, time::{self, Instant}, sync::{Arc, Mutex}, thread::spawn, fs::File, io::Write};

use argparse::{ArgumentParser, Store};
use rand::Rng;
use revsesi_chess_test_cases::chessboard::{ChessGame, random_select};

fn work(id: isize, map: Arc<Mutex<BTreeMap<String, f64>>>, count: usize) {
    let start = time::Instant::now(); 
    let d = ChessGame::default(); 
    let mut main_chess = ChessGame::default(); 
    let mut rng = rand::thread_rng(); 
    for _ in 0..count {
        let start = time::Instant::now(); 
        main_chess.set_as(&d); 
        for _a in 0..15 { if main_chess.is_end() { break } random_select(&mut main_chess, &mut rng); }
        let v = main_chess.to_string(); 
        // let k = ans.entry(v);
        let mut ans; 
        ans = map.as_ref().lock().unwrap(); 
        if ans.contains_key(&v) {
            eprintln!("\x1b[33;1m[thread {id}] meet an already exist key... \x1b[0m"); 
            continue 
        }
        ans.insert(v.clone(), 0.); 
        drop(ans); 
        let ans_value = calculate(&main_chess, &mut rng); 
        let end = time::Instant::now(); 
        let dur = end - start; 
        let millus = dur.as_millis() as f64 / 1e3; 
        eprintln!("\x1b[36;1m[thread {id}] batch data generate with {} s.\x1b[0m", millus); 
        ans = map.as_ref().lock().unwrap(); 
        let to_check = ans.insert(v, ans_value); 
        drop(ans); 
        assert_eq!(to_check, Some(0.)); 
    }
    let end = time::Instant::now() - start;
    eprintln!("\x1b[36;1m[thread {id}] total batch: {count} | total time cost: {} s. \x1b[0m", end.as_millis() as f64 / 1e3); 
}

fn main() {
    let mut target_file = "default_output.txt".to_string(); 

    let mut ap = ArgumentParser::new();
    ap.refer(&mut target_file).add_option(&["--file", "-f"], Store, "define the data output file name."); 
    ap.parse_args_or_exit(); 
    drop(ap); 

    let start = time::Instant::now(); 
    let ans = Arc::new(Mutex::new(BTreeMap::<String, f64>::default())); 
    let mut ts = Vec::new(); 
    for i in 0..8 {
        let p = ans.clone(); 
        let s = spawn(move || work(i, p, 100) );
        ts.push(s); 
    }
    for s in ts { s.join().unwrap() } 
    let end = Instant::now(); 
    let ct = end - start; 
    eprintln!("[thread core] total time cost: {} s.", ct.as_millis() as f64 / 1e3); 

    let mut f = File::create(target_file).expect("Open the file to output data. "); 
    for (s, &p) in ans.as_ref().lock().unwrap().iter() {
        writeln!(f, "{s}: {p}").unwrap(); 
    }
    let end = Instant::now(); 
    let ct = end - start; 
    eprintln!("[thread core] total time cost after data output: {} s.", ct.as_millis() as f64 / 1e3); 
}

fn calculate(this: &ChessGame, random_rng: &mut impl Rng) -> f64 {
    if this.is_end() { if this.player_index() == 1 { return 1. } else { return 0. } } 
    let mut base = ChessGame::default(); 
    const REPEATS: usize = 10_000; 
    let mut wins = 0usize; 
    for _ in 0..REPEATS {
        base.set_as(this); 
        while !base.is_end() {
            random_select(&mut base, random_rng); 
        }
        if base.is_end() {
            match base.player_index() {
                1 => wins += 1, 
                _ => (), 
            }
        }
    }
    wins as f64 / REPEATS as f64 
}