use std::mem::MaybeUninit;

use crate::random::{Random, F64Generate}; 

pub fn select_randomly_plus(your_turn: crate::ChessBoardGameState, board: &crate::ChessBoard::<8>, rng: &mut &mut impl Random<ResultType = u32>) -> Option<(i32, i32)> {
    let mut select = None; 
    let mut cnt: i32 = unsafe {
        MaybeUninit::uninit().assume_init()
    }; 

    let evaluate = |a, b| {
        assert! (a >= 0 && a < 8); 
        assert! (b >= 0 && b < 8); 
        let xcost = if a <= 3 {
            4 - a 
        } else {
            a - 3  
        }; 
        let ycost = if b <= 3 {
            4 - b 
        } else {
            b - 3 
        }; 
        9 - (xcost + ycost)
    }; 

    for i in 0..64 {
        let (x, y) = (i / 8, i % 8); 
        if board.attempt_apply_fast(your_turn, x, y).is_ok() {
            if let None = select {
                select = Some((x, y)); 
                cnt = evaluate(x, y); 
            } else {
                let e = evaluate(x, y); 
                cnt += e; 
                if rng.next_f64() <= 1. / cnt as f64 {
                    select = Some((x, y)) 
                }
            }
        }
    }

    select 
}