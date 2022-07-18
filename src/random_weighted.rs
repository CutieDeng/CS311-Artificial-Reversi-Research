const EVALUATE_MATRIX: [[i32; 8]; 8] = 
[
    [500, -25, 10, 5, 5, 10, -25, 500], 
    [-25, -45, 1, 1, 1, 1, -45, -25], 
    [10, 1, 3, 2, 2, 3, 1, 10], 
    [5, 1, 2, 1, 1, 2, 1, 5], 
    [5, 1, 2, 1, 1, 2, 1, 5], 
    [10, 1, 3, 2, 2, 3, 1, 10], 
    [-25, -45, 1, 1, 1, 1, -45, -25], 
    [500, -25, 10, 5, 5, 10, -25, 500],
]; 

use crate::random::{Random, F64Generate}; 
use core::mem::MaybeUninit;
use std::cmp::Ordering; 

pub fn select_weighted(your_turn: crate::ChessBoardGameState, board: &crate::ChessBoard::<8>, rng: &mut &mut impl Random<ResultType = u32>) -> Option<(i32, i32)> {
    let mut select = unsafe {
        MaybeUninit::uninit().assume_init()
    }; 
    let mut max: Option<i32> = None; 
    let mut cnt: isize = unsafe { MaybeUninit::uninit().assume_init() }; 
    let evaluate = |s| {
        let mut ans = 0; 
        for (x, y) in s {
            ans += EVALUATE_MATRIX[x as usize][y as usize] * 2; 
        }
        ans 
    }; 
    for i in 0..64 {
        let (x, y) = (i / 8, i % 8); 
        if let Some(l) = board.attempt_apply(your_turn, x, y).ok() {
            let cal = evaluate(l) + 
                EVALUATE_MATRIX[x as usize][y as usize]; 
            match max {
                Some(v) => {
                    match v.cmp(&cal) {
                        Ordering::Less => {
                            select = (x, y); 
                            max = Some(cal); 
                            cnt = 1; 
                        }
                        Ordering::Equal => {
                            cnt += 1; 
                            if rng.next_f64() < 1. / cnt as f64 {
                                select = (x, y); 
                                max = Some(cal); 
                            }
                        }
                        Ordering::Greater => (), 
                    }
                } 
                None => {
                    select = (x, y); 
                    max = Some(cal); 
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