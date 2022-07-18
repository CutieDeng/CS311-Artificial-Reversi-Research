use reversi::*; 

mod wins {
    use core::sync::atomic::AtomicIsize; 
    pub static BLACK: AtomicIsize = AtomicIsize::new(0); 
    pub static WHITE: AtomicIsize = AtomicIsize::new(0); 
}

fn main() {
    extern "C" {
        fn srand(_: u64); 
        fn time(_: usize) -> u64; 
    }
    unsafe {
        srand(time(0)); 
    }

    let mut threads = Vec::new(); 

    let start = reversi::get_millis_of_time().unwrap(); 

    if cfg!(feature = "multi-thread") {
        for _ in 0..20 {
            use std::thread; 
            threads.push(thread::spawn(|| {
                test_repeated(500000, &mut random::mt19937::Mt19937::new( unsafe {
                    time::time(core::ptr::null_mut())
                }.try_into().unwrap())); 
            })); 
        }
        for s in threads {
            s.join().unwrap(); 
        }
    } else {
        test_repeated(1, &mut random::empty_random::EmptyRandom::default()); 
    }

    let end = get_millis_of_time().unwrap(); 
    let diff = end.subtract(&start); 

    println!("Game over, cost time = {} ms. ", diff / 1000); 
    
    use core::sync::atomic::Ordering::Relaxed; 
    println!("Execute some games, the black wins {}, and the white wins {}. ", 
        wins::BLACK.load(Relaxed), wins::WHITE.load(Relaxed)); 
}

#[allow(dead_code)]
fn test() {
    let mut t: ChessBoard::<8> = Default::default(); 
    test_with_mut(&mut t, &mut random::empty_random::EmptyRandom::default()) 
}

fn test_repeated(repeated_times: u32, rng: &mut impl random::Random<ResultType = u32>) {
    let mut t: ChessBoard::<8> = Default::default(); 
    for _ in 0..repeated_times {
        t.reinit(); 
        test_with_mut(&mut t, rng); 
    }
}

fn test_with_mut (t: &mut ChessBoard::<8>, mut rng: &mut impl random::Random<ResultType = u32>) {
    // let mut t: ChessBoard::<8> = Default::default(); 
    if cfg!(feature = "debug_step") {
        println!("{}\n现在是黑棋下手时间", t); 
    }
    use ChessBoardGameState::*; 
    let mut now = BlackTurn;     
    let mut silent_time = 0; 
    while now != End {
        silent_time += 1; 
        let choose = if let BlackTurn = now {
            random_select::select_randomly(now, &t, &mut rng) 
        } else {
            random_weighted::select_weighted(now, &t, &mut rng)
        }; 
        if let Some(p) = choose {
            let _ = t.apply(now, p.0, p.1); 
            silent_time = 0; 
        }
        if cfg!(feature = "debug_step") {
            println!("{}\n", t); 
        }
        if now == BlackTurn {
            now = WhiteTurn; 
            if cfg!(feature = "debug_step") {
                println!("现在是白棋下手时间"); 
            }
        } else {
            now = BlackTurn; 
            if cfg!(feature = "debug_step") {
                println!("现在是黑棋下手时间"); 
            }
        }
        if silent_time >= 2 {
            now = End; 
        }
        if cfg!(debug) {
            extern "C" {
                fn sleep(_: i32); 
            }
            unsafe {sleep(1)}; 
        }
    }
    let (b, w) = t.count(); 
    use core::sync::atomic::Ordering::Relaxed; 
    use core::cmp::Ordering::*; 
    match b.cmp(&w) {
        Greater => {
            wins::WHITE.fetch_add(1, Relaxed);
        }
        Less => {
            wins::BLACK.fetch_add(1, Relaxed); 
        }
        Equal => (), 
    }
    if cfg!(feature = "debug_step") {
        println!("{}\n----------------", t); 
    }
}