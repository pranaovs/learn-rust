use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: get_input("width"),
        height: get_input("height"),
    };

    println!("The area of the rectangle is: {}", area(&rect));
}

fn get_input(inp: &str) -> u32 {
    println!("Enter value for {inp}: ");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Input a number");
                continue;
            }
        }
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
