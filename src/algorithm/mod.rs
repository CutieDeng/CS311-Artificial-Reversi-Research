use crate::random::Random; 

pub trait Player {
    fn play(your_turn: crate::ChessBoardGameState, board: &crate::ChessBoard::<8>, rng: &mut &mut impl Random<ResultType = u32>) -> Option<(i32, i32)>; 
}

pub mod random_select; 
pub mod greedy_select; 
pub mod weighted_select; 
pub mod random_plus; 