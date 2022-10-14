#![deprecated]
//! # The chessboard module 
//! 
//! 

pub struct Chess ([u64; 2]); 

impl Default for Chess {
    fn default() -> Self {
        let mut this = Chess([0; 2]); 
        this.set_color_purely(27, Some(Player::Black)); 
        this.set_color_purely(36, Some(Player::Black)); 
        this.set_color_purely(28, Some(Player::White)); 
        this.set_color_purely(35, Some(Player::White)); 
        this
    }
}

impl Chess {

    pub const EDGE_SIZE: usize = 8; 

    pub fn restart (&mut self) {
        *self = Default::default(); 
    }

    /// It would provides `panic` when index is larger than 64. 
    pub fn set_color_purely(&mut self, index: usize, color: Option<Player>) {
        Chess::check_index_with_panic(index); 
        let value: u64 = match color {
            Some(p) => {
                match p {
                    Player::Black => 1,
                    Player::White => 2,
                }
            }
            None => {
                0
            }
        }; 
        let to_change: &mut u64; 
        to_change = if index < 32 { &mut self.0[0] } else { &mut self.0[1] }; 
        let index = if index >= 32 { index - 32 } else { index }; 
        let mask: u64 = !(0x3u64 << (index * 2)); 
        *to_change &= mask; 
        #[cfg( feature = "cutie-optimize-level-1" )]
        if value == 0 { return } 
        *to_change |= value << (index * 2);
    }

    fn check_index_with_panic(index: usize) {
        assert! (index < Chess::EDGE_SIZE * Chess::EDGE_SIZE, "Invalid index {{row={}, col={}}}. ", index/Chess::EDGE_SIZE, index%Chess::EDGE_SIZE); 
    }

    pub fn select(&self, index: usize, ) -> Option<Player> {
        Chess::check_index_with_panic(index); 
        let (fetch, index) = if index < 32 { (&self.0[0], index) } else { (&self.0[1], index - 32) }; 
        let fetch = *fetch >> (index * 2); 
        let fetch = fetch & 0x3; 
        match fetch {
            0 => None, 
            0x1 => Some(Player::Black), 
            0x2 => Some(Player::White), 
            _ => unreachable!("fetch a piece on the chessboard, which is ill-formed code: 3. "), 
        }
    }

    

}

pub struct GameResult {
    is_end: bool, 
    now: Player, 
}

pub struct GameController {
    chess: Chess, 
    next_player: GameResult, 
}

impl Default for GameController {
    fn default() -> Self {
        Self { chess: Default::default(), next_player: GameResult { is_end: false, now: Player::Black } }
    }
}

impl GameController {
    pub const fn peek_chess(&self) -> &Chess {
        &self.chess
    }
}

impl GameController {
    pub fn next_randomly(&mut self, ) {

    }
}

#[derive(PartialEq, Eq)]
pub enum Player {
    Black, 
    White, 
}

impl Player {
    pub const fn is_black(&self) -> bool {
        if let Player::Black = *self {
            true 
        } else {
            false 
        }
    }

    pub const fn is_white(&self) -> bool {
        if let Player::White = *self {
            true 
        } else {
            false 
        }
    }
}