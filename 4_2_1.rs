/*
// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!");
}*/ 


// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); //Each character would occupy 4bits/1byte as 1 byte=4bits

    let c2 = '中';

    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 