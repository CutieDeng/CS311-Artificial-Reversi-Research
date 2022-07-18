pub trait Random {
    type ResultType; 
    fn next(&mut self) -> Self::ResultType; 
}

pub trait F64Generate {
    fn next_f64(&mut self) -> f64; 
}

impl <T: Random<ResultType = u32>> F64Generate for T {
    fn next_f64(&mut self) -> f64 {
        self.next() as f64 / u32::MAX as f64 
    }
}

pub mod minstd_rand0; 
pub mod minstd_rand; 
pub mod empty_random;
pub mod mt19937;