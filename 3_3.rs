
// Fix the error below with least amount of modification

/*fn main() {
    
let x: i32 = 10;
    
{
        
let y: i32 = 5;
        
println!("The value of x is {} and value of y is {}", x, y);
    
}
    
println!("The value of x is {} and value of y is {}", x, y); 
}*/



fn main() {
    
let x: i32 = 10;//Value 10 is binded to x which is variable of type i32
  
        
let y: i32 = 5;
        
println!("The value of x is {} and value of y is {}", x, y);//Anything within a scope({}) is not valid outside that scope
    

    println!("The value of x is {}and value of y is {}", x, y); 

}
