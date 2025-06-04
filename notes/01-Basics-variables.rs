fn immutable() {
    println!("Demonstrating immutable variables:");

    let x = 5; // Immutable variable
    println!("The value of x is: {}", x);
    /*
    x = 6; // This will cause a compile-time error because x is immutable
    println!("The value of x is now: {}", x); // This line will not be reached
    */
}

fn mutable() {
    println!("Demonstrating mutable variables:");

    let mut y = 10; // Mutable variable
    println!("The value of y is: {}", y);
    y = 15; // This is allowed because y is mutable
    println!("The value of y is now: {}", y);
}

fn constants() {}
    println!("Demonstrating constants:");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

fn shadowing()  {
    println!("Demonstrating variable shadowing:");

    let x = 5;
    let x = x + 1; // shadowing, x is now 6
    {
        let x = x * 2; // shadowing
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x in the outer scope is: {}", x); // 6
}

fn main() {
    immutable(); // Uncommenting this will cause a compile-time error
    mutable(); // This will run successfully
    constants(); // This will run successfully
    shadowing(); // This will run successfully
}
