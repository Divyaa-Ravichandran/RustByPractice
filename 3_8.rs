// Fix the error below with least amount of modification
/*fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}*/
// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);// either make x as mutable
    x += 2;

  // shadow value of x
  let x=3;
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
