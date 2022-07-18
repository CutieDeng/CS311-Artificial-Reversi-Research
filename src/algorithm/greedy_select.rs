use super::*; 

pub struct GreedySelect;

use core::mem::MaybeUninit; 
use core::cmp::Ordering; 

impl Player for GreedySelect {
    fn play(your_turn: crate::ChessBoardGameState, board: &crate::ChessBoard::<8>, rng: &mut &mut impl Random<ResultType = u32>) -> Option<(i32, i32)> {
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
}