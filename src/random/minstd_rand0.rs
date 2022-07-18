use super::Random; 
pub struct MinstdRand0 (usize); 

extern "C" {
    fn minstd_rand0(_: u32) -> usize; 
    fn delete_minstd_rand0(_: usize); 
    fn next_minstd_rand0(_: usize) -> u32; 
}

impl Random for MinstdRand0 {
    type ResultType = u32;
    fn next(&mut self) -> u32 {
        unsafe {
            next_minstd_rand0(self.0)
        } 
    }
}

impl Drop for MinstdRand0 {
    fn drop(&mut self) {
        unsafe {
            delete_minstd_rand0(self.0); 
        }
    }
}

impl MinstdRand0 {
    pub fn new (seed: u32) -> Self {
        Self (unsafe {
            minstd_rand0(seed)
        })
    }
}