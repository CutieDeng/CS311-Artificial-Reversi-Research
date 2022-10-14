use std::{io::{stdin, stdout, Write}, borrow::Cow};

use revsesi_chess_test_cases::{set_virtual_place, init, Config, Player, set_place};

fn main() {
    let mut buffer = [0; 64]; 
    let mut player = 1; 
    let mut game = [0u8; 64]; 
    let mut choose = None; 
    let mut specials = &buffer[0..0]; 
    let mut config = Config {
        black: Player { piece_exp: '\u{2713}', name: Cow::Borrowed("black"), place: Box::new(human_player)}, 
        white: Player { piece_exp: '\u{2717}', name: Cow::Borrowed("white"), place: Box::new(human_player)}, 
    }; 
    init(&mut game); 
    loop {
        dbg!(specials); 
        if let Some(v) = choose {
            config.display_std_out_extraly(&game, "", &[v], specials); 
        } else {
            config.display_std_out_extraly(&game, "", &[], specials); 
        }
        let f = if player == 1 {
            &mut config.black
        } else {
            &mut config.white
        }.place.as_mut()(&game, player); 
        if f == 64 {
            init(&mut game); 
            player = 1; 
            continue 
        } else if f == 65 {
            println!("exit! "); 
            break 
        }
        // choose = Some(f); 
        let p = set_virtual_place(&mut game, player, f, &mut buffer);
        if set_place(&mut game, player, f, false) {
            player = 3 - player; 
            choose = Some(f); 
        } else {
            println!("Cannot choose! "); 
        }
        specials = p; 
    }
}

fn read_u8<'a, T: Iterator<Item = &'a str>> (mut it: T) -> Result<u8, String> {
    let value; 
    if let Some(s) = it.next() {
        value = if let Ok(v) = s.parse::<u8>() {
            v
        } else {
            return Err("Invalid input at index 0. ".into()) 
        }
    } else {
        return Err("Too less arguments! ".into())
    }
    Ok(value)
}

fn human_player(_chesses: &[u8; 64], you: u8) -> i32 {
    let mut input = String::new(); 
    loop {
        input.clear(); 
        if you == 1 {
            print!("[black] > "); 
        } else if you == 2 {
            print!("[white] > "); 
        } else {
            unreachable!(); 
        }
        stdout().flush().unwrap(); 
        stdin().read_line(&mut input).unwrap(); 
        if input.trim_end() == "restart" {
            break 64
        } 
        if input.trim_end() == "exit" {
            break 65
        }
        let mut it = input.split_ascii_whitespace();
        let value1 = match read_u8(&mut it) {
            Ok(v) => {v},
            Err(e) => {
                println!("{}", e); 
                continue 
            },
        }; 
        let value2 = match read_u8(&mut it) {
            Ok(v) => {v},
            Err(e) => {
                println!("{}", e); 
                continue 
            },
        }; 
        if value1 >= 8 {
            println!("Invalid row index! "); 
            continue 
        }
        if value2 >= 8 {
            println!("Invalid col index! "); 
            continue 
        }
        break (value1 * 8 + value2).into() 
    }
}