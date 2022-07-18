use reversi::{time, random::Random};

fn main() {
    use reversi::random; 
    let mut r = random::minstd_rand0::MinstdRand0::new( unsafe {
        time::time(core::ptr::null_mut()).try_into().unwrap()
    });
    for _ in 0..20 {
        println!("Value is {}! ", r.next()); 
    }
    println!("随机数测试结束！"); 
}