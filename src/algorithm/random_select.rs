pub struct RandomSelect; 

use std::mem::MaybeUninit;

use crate::random::F64Generate;

use super::*; 

impl Player for RandomSelect {
    fn play(your_turn: crate::ChessBoardGameState, board: &crate::ChessBoard::<8>, rng: &mut &mut impl Random<ResultType = u32>) -> Option<(i32, i32)> {
        let mut cnt: i32 = unsafe {
            MaybeUninit::uninit().assume_init()
        }; 
        let mut select = None; 
        for i in 0..64 {
            let (x, y) = (i / 8, i % 8); 
            if board.attempt_apply_fast(your_turn, x, y).is_ok() {
                match select {
                    Some (_) => {
                        cnt += 1; 
                        if rng.next_f64() < 1. / cnt as f64 {
                            select = Some((x, y)) 
                        } 
                    }
                    None => {
                        cnt = 1; 
                        select = Some((x, y))
                    }
                }
            }
        }
        select 
    } 
}