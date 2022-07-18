use std::{mem::MaybeUninit, cmp::Ordering};

use crate::{self as reversi, random::Random}; 

pub fn select_randomly<T> (your_turn: reversi::ChessBoardGameState, board: &reversi::ChessBoard::<8>, rng: &mut &mut T) -> Option<(i32, i32)> 
    where T: Random<ResultType = u32> { 
    let mut attempt = Vec::with_capacity(32); 
    for i in 0..64 {
        let (x, y) = (i / 8, i % 8); 
        if board.attempt_apply_fast(your_turn, x, y).is_ok() {
            attempt.push((x, y)); 
        }
    }
    if attempt.is_empty() {
        None 
    } else {
        let select_seed = rng.next(); 
        Some(attempt[select_seed as usize % attempt.len()])
    }
}

pub fn select_greedy(your_turn: reversi::ChessBoardGameState, board: &reversi::ChessBoard::<8>, rng: &mut &mut impl Random<ResultType = u32>) -> Option<(i32, i32)> {
    let mut select = unsafe {
        MaybeUninit::uninit().assume_init()
    }; 
    let mut max: Option<usize> = None; 
    let mut cnt: isize = unsafe { MaybeUninit::uninit().assume_init() }; 
    for i in 0..64 {
        let (x, y) = (i / 8, i % 8); 
        if let Some(l) = board.attempt_apply(your_turn, x, y).ok() {
            match max {
                Some(v) => {
                    // if v > l.len() {
                    //     select = (x, y); 
                    //     max = Some(l.len())
                    // } 
                    match v.cmp(&l.len()) {
                        Ordering::Greater => {
                            select = (x, y); 
                            max = Some(l.len()); 
                            cnt = 1; 
                        }
                        Ordering::Equal => {
                            cnt += 1; 
                            let p = rng.next() as f64 / u32::MAX as f64; 
                            let critical = 1. / cnt as f64; 
                            if p <= critical {
                                select = (x, y); 
                                max = Some(l.len()); 
                            }
                        }
                        Ordering::Less => (), 
                    }
                }
                None => {
                    select = (x, y); 
                    max = Some(l.len());
                    cnt = 1; 
                }
            }
        }
    }
    if let None = max {
        None 
    } else {
        Some(select)
    }
}