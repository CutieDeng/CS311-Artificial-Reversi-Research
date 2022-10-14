use std::{collections::BTreeMap, time};

use rand::Rng;
use revsesi_chess_test_cases::chessboard::{ChessGame, random_select};

fn main() {
    let start = time::Instant::now(); 
    let mut ans = BTreeMap::<String, f64>::default(); 
    const VALUE: usize = 100; 
    let d = ChessGame::default(); 
    let mut main_chess = ChessGame::default(); 
    let mut rng = rand::thread_rng(); 
    for _ in 0..VALUE {
        let start = time::Instant::now(); 
        main_chess.set_as(&d); 
        for _a in 0..15 { if main_chess.is_end() { break } random_select(&mut main_chess, &mut rng); }
        let v = main_chess.to_string(); 
        // let k = ans.entry(v);
        if ans.contains_key(&v) {
            println!("\x1b[31;1mRepeat game state view...\x1b[0m"); 
            continue 
        }
        let ans_value = calculate(&main_chess, &mut rng); 
        let end = time::Instant::now(); 
        let dur = end - start; 
        let millus = dur.as_millis() as f64 / 1e3; 
        eprintln!("[time-cost: {millus} s.] {}: {}", v, ans_value); 
        let _ = ans.insert(v, ans_value); 
    }
    for (s, v) in ans {
        println!("{}: {}", s, v); 
    }
    let end = time::Instant::now() - start;
    eprintln!("Total time cost: {} s. ", end.as_millis() as f64 / 1e3); 
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