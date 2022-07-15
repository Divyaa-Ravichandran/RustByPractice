
fn main() {
   let v = {let x = 3;
           x};// put into { } and return x value within {}

   assert!(v == 3);

   println!("Success!");
}