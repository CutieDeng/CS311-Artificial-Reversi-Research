use std::{borrow::Cow, fmt::Write};

pub struct Config {
    pub black: Player, 
    pub white: Player, 
}

impl Config {
    pub fn display_std_out(&self, chesses: &[u8; 64], tip: &str) {
        self.display_std_out_extraly(chesses, tip, &[], &[]) 
    }

    pub fn display_std_out_extraly(&self, chesses: &[u8; 64], tip: &str, emphasized: &[i32], special: &[i32]) {
        if !tip.is_empty() {
            println!("{}", tip); 
        }
        for _ in 0..9 {
            print!("\u{2610} "); 
        }
        println!("\u{2610}"); 
        for i in 0..8 {
            print!("\u{2610}");
            for j in 0..8 {
                print!(" "); 
                let f = chesses[i*8+j]; 
                let index = i * 8 + j; 
                let e = emphasized.contains(&(index as i32));
                let s = special.contains(&(index as i32)); 
                if e {
                    print!("\x1b[31;1m"); 
                } else if s {
                    print!("\x1b[93;1m"); 
                }
                if f == 0 {
                    print!(" ");
                } else if f == 1 {
                    // black. 
                    print!("{}", self.black.piece_exp); 
                } else if f == 2 {
                    // white 
                    print!("{}", self.white.piece_exp); 
                } else {
                    unreachable!(); 
                }
                if e || s {
                    print!("\x1b[0m"); 
                }
            }
            println!(" \u{2610}"); 
        }
        for _ in 0..9 {
            print!("\u{2610} "); 
        }
        println!("\u{2610}"); 
    }
}

pub struct Player {
    pub piece_exp: char, 
    pub name: Cow<'static, str>, 
    pub place: Box<dyn FnMut(&[u8; 64], u8) -> i32>, 
}

pub enum StepResult {
    Win(u8), 
    Continue(u8), 
}

pub fn attempt_place<'a>(chesses: &[u8; 64], player: u8, buffer: &'a mut [i32; 64]) -> &'a[i32] {
    let mut length = 0usize; 
    let fetch = move |r: usize, c: usize| chesses[r * 8 + c]; 
    for i in 0..8 {
        for j in 0..8 {
            if fetch(i, j) == 0 {
                // empty position~ 
                let mut x; 
                let mut y; 
                let mut flag = false; 
                'movements_loop:
                for move_x in -1..2 {
                    for move_y in -1..2 {
                        if move_x == 0 && move_y == 0 {
                            continue 
                        }
                        x = i as i32; 
                        y = j as i32; 
                        let mut tag = false; 
                        loop {
                            x += move_x; 
                            y += move_y; 
                            if x < 0 || x >= 8 {
                                break 
                            }
                            if y < 0 || y >= 8 {
                                break 
                            }
                            let x = x as usize; let y = y as usize; 
                            let f = fetch(x, y); 
                            if f == 0 { 
                                break 
                            }
                            if f == player {
                                if tag {
                                    flag = true; 
                                    break 'movements_loop
                                } else {
                                    break 
                                }
                            } else {
                                tag = true; 
                            }
                        }
                    }
                }
                if flag {
                    buffer[length] = (i * 8 + j) as i32; 
                    length += 1; 
                }
            }
        }
    }
    &buffer[0..length]
}

pub fn set_virtual_place<'a>(chesses: &mut [u8; 64], player: u8, choose: i32, buffer: &'a mut [i32; 64]) -> &'a[i32] {
    let mut length = 0usize; 
    let fetch = move |r: usize, c: usize| chesses[r * 8 + c]; 
    for move_x in -1..=1 {
        for move_y in -1..=1 {
            if move_x == 0 && move_y == 0 {
                continue 
            }
            let mut x = choose / 8; 
            let mut y = choose % 8; 
            let mut tmp_size = length; 
            let mut success = false; 
            let mut flag = false; 
            loop {
                x += move_x; 
                y += move_y; 
                if x < 0 || x >= 8 { break }
                if y < 0 || y >= 8 { break }
                let x = x as usize; 
                let y = y as usize; 
                let f = fetch(x, y); 
                if f == 0 {
                    break
                }
                if f == player {
                    success = flag; 
                    break
                } else {
                    buffer[tmp_size] = (x * 8 + y) as i32; 
                    tmp_size += 1; 
                    flag = true; 
                }
            }
            if success {
                length = tmp_size; 
            }
        }
    } 
    &buffer[0..length]
}

pub fn set_place(chesses: &mut [u8; 64], player: u8, choose: i32, forcely: bool) -> bool {
    // let fetch = move |r: usize, c: usize| chesses[r * 8 + c]; 
    let f = chesses[choose as usize]; 
    if f != 0 {
        return false
    } 
    let mut cache = [0; 64]; 
    let result = set_virtual_place(chesses, player, choose, &mut cache); 
    if !forcely {
        if result.len() == 0 {
            return false 
        }
    }
    for &i in result {
        chesses[i as usize] = player; 
    }
    chesses[choose as usize] = player; 
    true 
}

pub fn write_chesses<T: Write>(fmt: &mut T, chesses: &[u8; 64]) {
    for &i in chesses {
        write!(fmt, "{}", i).unwrap(); 
    }
}

pub fn init(chesses: &mut [u8; 64]) {
    for i in 0..8 { 
        for j in 0..8 {
            chesses[i*8+j] = 0; 
        }
    }
    chesses[3*8+4] = 1; 
    chesses[4*8+3] = 1; 
    chesses[4*8+4] = 2; 
    chesses[3*8+3] = 2; 
}

#[test] 
fn empty_write_test() {
    let mut cached = String::new(); 
    let empty = [0u8; 64]; 
    write_chesses(&mut cached, &empty);
    assert_eq!(cached, "0".repeat(64));
}

#[test] 
fn init_write_test() {
    let mut result = String::new(); 
    let mut initialized = [0u8; 64]; 
    init(&mut initialized); 
    write_chesses(&mut result, &initialized); 
    assert_eq!(result, "\
    00000000\
    00000000\
    00000000\
    00021000\
    00012000\
    00000000\
    00000000\
    00000000"); 
}

pub mod chessboard; 