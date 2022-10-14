use std::{ops::{Deref, DerefMut}, borrow::Cow, fmt::{Display, Debug}};

use rand::Rng;

/// 描述了黑白棋棋盘本身数据
/// A proxy class for the chessboard. 
/// 
/// --- 
/// 
/// ### 有效编码范围
/// 
/// 其中元素数组有效值如下：
/// - 0: 当前位置为空
/// - 1: 当前位置为黑色棋子所在
/// - 2: 当前位置为白色棋子所在
#[derive(Clone)]
pub struct ChessBoard ( Box<[[u8; 8]; 8]> ); 

/// ChessBoard 棋盘代理内容暴露
impl Deref for ChessBoard {
    type Target = [[u8; 8]; 8]; 

    fn deref(&self) -> &Self::Target {
        &self.0 
    }
}

impl DerefMut for ChessBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for ChessBoard {
    /// ### ChessBoard 的默认初始化规则 
    /// 
    /// 居中样式：
    /// 
    /// 白 黑
    /// 黑 白
    fn default() -> Self {
        let mut this = Self(Default::default()); 
        this[3][4] = 1; 
        this[4][3] = 1; 
        this[4][4] = 2; 
        this[3][3] = 2; 
        this 
    }
}

impl ChessBoard {
    /// 测试棋盘上某个点 activate 后对某个方向的 influence. 
    /// 
    /// --- 
    /// 
    /// 这是一个基本的函数调用，设计常用于对实际的棋盘变化的封装：
    /// 
    /// 1. 当有人对空白点进行落子
    /// 1. 在棋盘上对空白点进行更新，以得到落子后的立即结果
    /// 1. 通过 [`attempt_roll`] 函数自身调用得到该过程可能带来的棋盘变化
    /// 1. 根据变化确定原位置落子是否合法
    ///
    /// # Examples
    ///
    /// ```
    /// use revsesi_chess_test_cases::chessboard::ChessBoard;
    ///
    /// let chess_board: ChessBoard = Default::default();
    /// let now_index = (3, 3); 
    /// let move_index = (None, Some(true)); 
    /// let result = chess_board.attempt_roll(now_index, move_index);
    /// assert_eq!(result, 0);
    /// ```
    /// 
    /// # Panics 
    /// 
    /// 1. move_index 传入了一个无效的值 (None, None) 
    /// 1. now_index 描述的棋子区间超出了预期
    /// 1. now_index 描述的位置并没有初始化棋子
    pub fn attempt_roll(&self, mut now_index: (usize, usize), move_index: (Option<bool>, Option<bool>)) -> usize {
        let origin_flag = self[now_index.0][now_index.1]; 
        assert ! (origin_flag != 0, "Cannot derive the related info ");
        assert ! (move_index.0.is_some() || move_index.1.is_some(), "Never movement happens in [attempt_roll] function call. "); 
        let mut result: usize = 0; 
        loop {
            if let Some(move_index) = move_index.0 {
                if move_index {
                    if now_index.0 == 7 {
                        break 0
                    } 
                    now_index.0 += 1; 
                } else {
                    if now_index.0 == 0 {
                        break 0
                    }
                    now_index.0 -= 1; 
                }
            }
            if let Some(move_index) = move_index.1 {
                if move_index {
                    if now_index.1 == 7 {
                        break 0
                    }
                    now_index.1 += 1; 
                } else {
                    if now_index.1 == 0 {
                        break 0
                    }
                    now_index.1 -= 1; 
                }
            }
            let now_flag = self[now_index.0][now_index.1]; 
            if now_flag == 0 { break 0 }
            if now_flag == origin_flag {
                break result 
            } else {
                result += 1; 
            }
        } 
    }

}

#[derive(Clone)]
pub struct ChessGame (ChessBoard, bool, u8); 

impl Default for ChessGame {
    /// ChessGame 游戏初始化
    /// 
    /// 初始化信息：初始化棋盘，开局为黑棋落子
    fn default() -> Self {
        Self (Default::default(), false, 1) 
    }
}

impl ChessGame {

    /// 获取当前的游戏玩家
    pub const fn player_index(&self) -> u8 {
        self.2 
    }

    pub const fn is_end(&self) -> bool { self.1 }

    /// 直接修改当前进行游戏的游戏玩家
    /// 
    /// # unsafe 
    /// 
    /// 可能致使游戏进入不一致的信息状态
    pub unsafe fn set_player(&mut self, new_player_index: u8) {
        assert! (new_player_index == 1 || new_player_index == 2, 
            "Unexpected player index set: {}", new_player_index); 
        self.2 = new_player_index; 
    }

    /// 选择下一步要下的位置，并推动游戏进入下一回合
    /// 
    /// # Error 
    /// 
    /// 非法的位置选择：index error... 
    /// 
    /// 该函数会推进 [`ChessGame`] 的进程
    pub fn set(&mut self, row: usize, col: usize) -> Result<(), Cow<'static, str>> {
        if self.1 {
            return Err(Cow::Borrowed("Game has ended."))
        }
        if row >= 8 {
            return Err(Cow::Borrowed("Row index shouldn't be larger than 8."))
        }
        if col >= 8 {
            return Err (Cow::Borrowed("Column index shouldn't be larger than 8. ")) 
        }
        if self.0[row][col] != 0 {
            return Err (Cow::Borrowed("The specific piece is occupied by someone. "))
        }
        let mut success = false; 
        self.0[row][col] = self.2; 
        for m in Self::MOVEMENTS {
            let cnt = self.0.attempt_roll((row, col), m); 
            if cnt == 0 { continue }
            success = true; 
            let (mut a, mut b) = (row, col); 
            for _ in 0..cnt {
                if let Some(b) = m.0 {
                    if b { a += 1 } else { a -= 1 } 
                }
                if let Some(p) = m.1 {
                    if p { b += 1 } else { b -= 1 } 
                }
                self.0[a][b] = self.2; 
            }
        }
        if !success {
            self.0[row][col] = 0; 
            return Err(Cow::Borrowed("Invalid piece - cannot flip any piece. "))
        }
        self.2 = 3 - self.2; 
        {
            let mut cached: [(usize, usize); 1] = Default::default(); 
            let r = self.find_all_valuables(&mut cached);
            if r.is_empty() {
                self.2 = 3 - self.2; 
                let r = self.find_all_valuables(&mut cached); 
                if r.is_empty() {
                    // 游戏结束！
                    self.deduce_winner(); 
                } 
            } 
        }
        Ok(())
    }

    fn deduce_winner(&mut self) {
        assert ! (!self.1, "Cannot deduce the finished game. "); 
        let mut count = [0usize; 3]; 
        for i in self.0.iter() {
            for &j in i {
                count[j as usize] += 1; 
            }
        }
        self.1 = true; 
        self.2 = match count[1].cmp(&count[2]) {
            std::cmp::Ordering::Less => 1, 
            std::cmp::Ordering::Equal => 0, 
            std::cmp::Ordering::Greater => 2, 
        }; 
    }

    /// 八种基本的移动情形
    const MOVEMENTS: [(Option<bool>, Option<bool>); 8] = [
        (Some(true), None), (Some(true), Some(true)), 
        (None, Some(true)), (None, Some(false)), 
        (Some(false), Some(false)), (Some(false), None), 
        (Some(false), Some(true)), (Some(true), Some(false))]; 

    /// 尽管这个函数使用了 mut 引用，但依旧保证程序运行结束前变化会被恢复
    /// 
    /// 而在此过程中，程序会试着找到对当前的玩家合法的位置并将其置入第二个参数（答案缓冲池）中。
    /// 
    /// # Panic 
    /// 
    /// 在棋局结束时请求该询问
    pub fn find_all_valuables<'a> (&mut self, ans: &'a mut [(usize, usize)]) -> &'a [(usize, usize)] {
        assert! (!self.1, "Find all valuable positions in a finished chessboard. "); 
        let mut cnt : usize = 0; 
        for i in 0..8 {
            for j in 0..8 {
                if self.0[i][j] != 0 { continue } 
                self.0[i][j] = self.2; 
                let mut success = false; 
                for m in Self::MOVEMENTS {
                    if self.0.attempt_roll((i, j), m) != 0 {
                        success = true; 
                        break 
                    }
                }
                if success {
                    if cnt < ans.len() {
                        ans[cnt] = (i, j); 
                        cnt += 1; 
                    }  
                    if cnt >= ans.len() {
                        self.0[i][j] = 0; 
                        break 
                    }
                }
                self.0[i][j] = 0; 
            }
        }
        &ans[0..cnt]
    }

    /// 简单的格式化
    pub fn to_string(&self) -> String {
        let mut ans = String::with_capacity(128); 
        for i in self.0.iter() {
            for v in i {
                ans.push_str(
                    match *v {
                        0 => "00", 
                        1 => "01", 
                        2 => "10", 
                        _ => unreachable!()
                    }
                )
            }
        }
        ans 
    }

    /// 将当前的棋局重设为目标棋局
    pub fn set_as(&mut self, other: &Self ) {
        self.0.0.copy_from_slice(&other.0[..]); 
        self.1 = other.1; 
        self.2 = other.2; 
    }

}

pub fn random_select(chess_game: &mut ChessGame, rng: &mut impl Rng) {
    let mut c = [(0usize, 0usize); 64]; 
    let c = chess_game.find_all_valuables(&mut c); 
    debug_assert ! (!c.is_empty(), "Random select but the selection is empty! The chessboard info:\n{:+}", 
        chess_game); 
    let r = rng.gen_range(0..c.len());
    chess_game.set(c[r].0, c[r].1).unwrap(); 
}

impl Display for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            for r in self.0.iter() {
                for (j, v) in r.iter().enumerate() {
                    if j != 0 { write! (f, " ")? }
                    write!(f, "{}", match *v { 0 => " ", 1 => "+", 2 => "-", _ => unreachable!(), })? 
                }
                writeln!(f, "")? 
            }
            return Ok(())
        }
        todo!()
    }
}

impl Debug for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}", self) 
    }
}