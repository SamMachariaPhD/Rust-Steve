fn main() {
    /*
    let x = 7;
    println!("The value is {}", x);
    x = 3; // There will be an error here coz x is immutable.
    println!("The value has changed to {}", x);
    */
    let mut x = 7;
    println!("The value is {}", x);
    x = 3; // No error coz x is mutable
    println!("The value has changed to {}", x);
    /*
    let x = 7;
    println!("The value is {}", x);
    let x = 3; // I realized there's no error if you redeclare.
    println!("The value has changed to {}", x);
    */
}
