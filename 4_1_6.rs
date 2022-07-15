// Modify `assert!` to make it work
/*fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");
}*/


// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
// when we add the numbers they sum upto 1579
    println!("Success! {} {} {} {}",1_024,0xff,0o77,0b1111_1111);
  
}
