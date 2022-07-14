// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
/*fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x =  42;
    println!("{}", x); // Prints "42".
}*/

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);// x is compared to 12 within the scope {}
    }

    assert_eq!(x, 5);//x is compared to 5 outside the scope {} where x is having value as 5

    let x =  42;
    println!("{}", x); // Prints "42".
}