extern "C" {
    fn rand() -> u32; 
}

use std::marker::PhantomData; 
pub struct EmptyRandom (PhantomData<EmptyRandom>); 

use super::Random; 

impl Random for EmptyRandom {
    type ResultType = u32;
    fn next(&mut self) -> Self::ResultType {
        unsafe {
            rand() 
        } 
    }
}

impl Default for EmptyRandom {
    fn default() -> Self {
        EmptyRandom(PhantomData)
    }
}