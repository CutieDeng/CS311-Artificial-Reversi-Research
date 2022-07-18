use std::{fmt::Display, };

#[derive(Clone, Copy)]
pub enum ChessBoardState {
    Black, 
    White, 
    None, 
}

#[derive(Copy, Clone, PartialEq)]
pub enum ChessBoardGameState {
    BlackTurn, 
    WhiteTurn, 
    End, 
}

pub enum GamePlay {
    Black, 
    White, 
}

impl Default for ChessBoardState {
    fn default() -> Self {
        ChessBoardState::None
    }
}

#[derive(Clone)]
pub struct ChessBoard < const LENGTH: usize > {    
    boards: Vec<ChessBoardState>, 
}

impl < const L: usize > ChessBoard<L>{
    pub fn new() -> Self {
        let mut a = Self {
            boards: Vec::new(), 
        }; 
        a.boards.resize_with(L * L, Default::default); 
        a
    }

    pub fn count(&self) -> (isize, isize) {
        let (mut b, mut w) = (0isize, 0isize); 
        for &m in &self.boards {
            match m {
                ChessBoardState::Black => b += 1, 
                ChessBoardState::White => w += 1, 
                ChessBoardState::None => (), 
            } 
        }
        (b, w) 
    }
}

pub const STD_LOVE: char = '\u{2665}'; 
pub const ITALIC_LOVE: char = '\u{2766}'; 
pub const SOLID_SPADE: char = '\u{2660}'; 
pub const HOLLOW_HEART: char = '\u{2661}'; 

impl < const L: usize > Display for ChessBoard<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cache = String::with_capacity(L * (L + 1)); 
        for col in 0..L {
            for row in 0..L {
                use ChessBoardState::*; 
                match self.boards[row * L + col] {
                    None => cache.push_str("  "), 
                    White => 
                        cache.push(HOLLOW_HEART), 
                    Black => 
                        cache.push(SOLID_SPADE), 
                }
                if row + 1 == L {
                    cache.push('\n')
                } else {
                    cache.push(' ')
                }
            }
        }
        f.write_str(&cache)
    }
}

impl < const L: usize > Default for ChessBoard<L> {
    fn default() -> Self {
        let mut a = Self::new(); 
        assert! (L % 2 == 0, "The constant generic argument should be a multiple of 2, but actually it's {L}. "); 
        assert! (L > 0, "The constant generic argument should be positive. "); 
        let mid_value: usize = L / 2 - 1; 
        a.boards[ mid_value * L + mid_value ] = ChessBoardState::Black; 
        a.boards[ mid_value * L + mid_value + 1 ] = ChessBoardState::White; 
        a.boards[ (mid_value + 1) * L + mid_value ] = ChessBoardState::White; 
        a.boards[ (mid_value + 1) * L + (mid_value + 1) ] = ChessBoardState::Black; 
        a
    }
}

const MOVE_DIRECTIONS: [(i32, i32); 8] = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)]; 

impl < const L: usize > ChessBoard<L> {

    pub fn attempt_apply_fast (&self, your_turn: ChessBoardGameState, x: i32, y: i32) -> Result<(), String> {
        if x < 0 || x as usize >= L {
            return Err(format!("非法的 x 坐标落子位置：{}", x)); 
        }
        if y < 0 || y as usize >= L {
            return Err(format!("非法的 y 坐标落子位置：{}", y)); 
        }
        use ChessBoardGameState::*; 
        let is_black = match your_turn {
            End => return Err("游戏结束无法落子".into()), 
            BlackTurn => true, 
            WhiteTurn => false, 
        }; 
        use ChessBoardState::*; 
        if match self.boards[x as usize * L + y as usize] {
            None => false, 
            _ => true, 
        } {
            return Err("该位置已有子目，无法落子".into()); 
        }
        for direct in MOVE_DIRECTIONS {
            let (mut x, mut y) = (x, y); 
            let mut valid = false; 
            loop {
                x += direct.0; 
                y += direct.1; 
                if x < 0 || x >= L as i32 {
                    break 
                }
                if y < 0 || y >= L as i32 {
                    break 
                }
                let is_black_this_block = match self.boards[x as usize * L + y as usize] {
                    None => {
                        break; 
                    } 
                    Black => true, 
                    White => false, 
                }; 
                if is_black ^ is_black_this_block {
                    valid = true; 
                } else {
                    // result.extend_from_slice(&cache); 
                    if valid {
                        return Ok(()) 
                    } else {
                        break 
                    }
                }
            }
        } 
        Err("没有可转化的棋子".into())
    }

    pub fn attempt_apply (&self, your_turn: ChessBoardGameState, x: i32, y: i32) -> Result<Vec<(i32, i32)>, String> {
        if x < 0 || x as usize >= L {
            return Err(format!("非法的 x 坐标落子位置：{}", x)); 
        }
        if y < 0 || y as usize >= L {
            return Err(format!("非法的 y 坐标落子位置：{}", y)); 
        }
        use ChessBoardGameState::*;
        let is_black = match your_turn {
            End => return Err("游戏结束无法落子".into()), 
            BlackTurn => true, 
            WhiteTurn => false, 
        }; 
        use ChessBoardState::*; 
        if match self.boards[x as usize * L + y as usize] {
            None => false, 
            _ => true, 
        } {
            return Err("该位置已有子目，无法落子".into()) 
        }
        let mut cache = Vec::with_capacity(L); 
        let mut result = Vec::with_capacity(L * 3); 
        for direct in MOVE_DIRECTIONS {
            let (mut x, mut y) = (x, y); 
            loop {
                x += direct.0; 
                y += direct.1; 
                if x < 0 || x >= L as i32 {
                    break 
                }
                if y < 0 || y >= L as i32 {
                    break 
                }
                // if let None = self.boards[x * L + y] {
                //     cache.clear(); 
                //     break; 
                // } 
                let is_black_this_block = match self.boards[x as usize * L + y as usize] {
                    None => {
                        cache.clear(); 
                        break; 
                    } 
                    Black => true, 
                    White => false, 
                }; 
                if is_black ^ is_black_this_block {
                    cache.push((x, y)); 
                } else {
                    // result.extend_from_slice(&cache); 
                    result.append(&mut cache); 
                    debug_assert! (cache.is_empty()); 
                    // cache.clear(); 
                    break; 
                }
            }
        } 
        if !result.is_empty() {
            result.push((x, y)); 
            Ok(result)
        } else {
            Err("没有可转化的棋子".into())
        }
    }

    pub fn apply (&mut self, your_turn: ChessBoardGameState, x: i32, y: i32) -> Result<Vec<(i32, i32)>, String> {
        let ans = self.attempt_apply(your_turn, x, y)?; 
        for (x, y) in &ans {
            self.boards[*x as usize * L + *y as usize] = if let ChessBoardGameState::BlackTurn = your_turn {
                ChessBoardState::Black
            } else {
                ChessBoardState::White
            }; 
        } 
        Ok(ans) 
    }

    pub fn get(&self, a: i32, b: i32) -> &ChessBoardState {
        assert! (a >= 0 && (a as usize) < L); 
        assert! (b >= 0 && (b as usize) < L); 
        &self.boards[a as usize * L + b as usize] 
    }

    pub fn reinit(&mut self) {
        self.boards.fill(ChessBoardState::None); 
        let mid_value: usize = L / 2 - 1; 
        self.boards[ mid_value * L + mid_value ] = ChessBoardState::Black; 
        self.boards[ mid_value * L + mid_value + 1 ] = ChessBoardState::White; 
        self.boards[ (mid_value + 1) * L + mid_value ] = ChessBoardState::White; 
        self.boards[ (mid_value + 1) * L + (mid_value + 1) ] = ChessBoardState::Black; 
    }

    pub fn set(&mut self, other: &mut Self) {
        let (mut l, mut r) = (self.boards.iter_mut(), other.boards.iter_mut()); 
        while let (Some(l), Some(r)) = (l.next(), r.next()) {
            *l = *r   
        }
    }

}

pub mod time; 
pub mod random; 

pub use time::get_millis_of_time; 

pub mod display_array; 

pub mod random_select; 
pub mod random_plus;
pub mod random_weighted; 
pub mod algorithm;