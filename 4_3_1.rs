// Make it work with two ways
/*fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}*/

// Make it work with two ways

// FIRST WAY
/*fn main() {
   let v = {
       let mut x = 1;
       x += 2;
     x// return value out of this scope
   };

   assert_eq!(v, 3);

   println!("Success!");
}*/

// SECOND WAY

// Make it work with two ways
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, ());// return Unit method

   println!("Success!");
}