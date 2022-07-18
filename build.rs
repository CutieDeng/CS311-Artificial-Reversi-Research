fn main() {
    println!("cargo:rustc-link-lib=rand"); 
    println!("cargo:rustc-link-lib=c++"); 
    println!("cargo:rustc-link-search=."); 
}