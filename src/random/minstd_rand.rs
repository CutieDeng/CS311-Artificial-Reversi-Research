use super::Random;

extern "C" {
    fn minstd_rand(_: u32) -> usize; 
    fn delete_minstd_rand(_: usize); 
    fn next_minstd_rand(_: usize) -> u32; 
}

pub struct MinstdRand(usize); 

impl MinstdRand {
    pub fn new(seed: u32) -> Self {
        Self (
            unsafe {
                minstd_rand(seed)
            }
        )
    }
}

impl Drop for MinstdRand {
    fn drop(&mut self) {
        unsafe {
            delete_minstd_rand(self.0)
        }
    }
}

impl Random for MinstdRand {
    type ResultType = u32;
    fn next(&mut self) -> u32 {
        unsafe { next_minstd_rand(self.0) }
    }
}
