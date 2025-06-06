use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn build() -> Self {
        let width = get_input("width");
        let height = get_input("height");
        Self::new(width, height)
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::build();
    // dbg!(&rect);

    // println!("The rectangle is: {rect:#?}");

    // println!("The area of the rectangle is: {}", area(&rect));

    println!("The area of the rectangle is: {}", rect.area());
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
