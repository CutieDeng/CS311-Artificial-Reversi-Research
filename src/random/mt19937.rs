use super::Random;

pub struct Mt19937 (usize); 

extern "C" {
    fn mt19937(_: u32) -> usize; 
    fn delete_mt19937(_: usize); 
    fn next_mt19937(_: usize) -> u32; 
}

impl Mt19937 {
    pub fn new(seed: u32) -> Mt19937 {
        Mt19937(unsafe {
            mt19937(seed)
        })
    }
}

impl Drop for Mt19937 {
    fn drop(&mut self) {
        unsafe {
            delete_mt19937(self.0)
        }
    }
}

impl Random for Mt19937 {
    type ResultType = u32;
    fn next(&mut self) -> Self::ResultType {
        unsafe {
            next_mt19937(self.0)
        }
    }
}