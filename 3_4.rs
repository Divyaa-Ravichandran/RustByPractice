// Fix the error with the use of define_x
/*fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}*/

fn main() {
    println!("{}, world", define_x()); //calling the function -define_x 
}
}

fn define_x() -> &'static str{//Specifying the return type of the function, which can be used later
    let x = "hello";
  x // returning the value of x
}