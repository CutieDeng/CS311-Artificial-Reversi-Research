#![allow(dead_code)]
use core::fmt;

struct VecWrapper <'a, T: fmt::Display> (&'a Vec<T>);

impl <'a, T: fmt::Display> Into<VecWrapper<'a, T>> for &'a Vec<T> {
    fn into(self) -> VecWrapper<'a, T> {
        VecWrapper (self)     
    }
}

impl <'a, T: fmt::Display> fmt::Display for VecWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new(); 
        str.push('['); 
        let mut first = false; 
        for i in self.0 {
            if first {
                str.push_str(", "); 
            }
            // str.push_str(i.into()); 
            str.push_str(&format!("{}", *i)); 
            first = true; 
        }
        str.push(']'); 
        f.write_str(&str)  
    }
}

struct DisplayArray; 

impl DisplayArray {
    pub fn display<'a, T> (v: VecWrapper<'a, T>) -> String where T: fmt::Display {
        format!("{}", v) 
    }
}

#[test] 
fn test_simple_int_ar() {
    let x = vec![1, 2, 3]; 
    assert_eq!(DisplayArray::display((&x).into()), "[1, 2, 3]"); 
}