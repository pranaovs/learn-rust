# Structs

A struct is a composite data type that groups together variables (fields) under a single name.
Structs are useful for representing complex data structures in a more organized way.

Unlike tuples, structs allow you to define named fields, making the code more readable and maintainable.

- Keyword: `struct`

```rust
struct Point {
  x: f64,
  y: f64,
}

fn main() {
  let p = Point { x: 1.0, y: 2.0 };
  println!("Point coordinates: ({}, {})", p.x, p.y);
}
```

Structs can be mutable or immutable. By default, fields are immutable unless specified otherwise.

```rust
fn main() {
  let mut p = Point { x: 1.0, y: 2.0 };
  p.x = 3.0; // This is allowed because `p` is mutable
  println!("Updated Point coordinates: ({}, {})", p.x, p.y);
}
```

## Field init shorthand

When the field name is the same as the variable name, you can use field init shorthand to simplify struct initialization.

```rust
fn main() {
  let point = build_point(1.0, 2.0);
}

fn build_point(x: f64, y: f64) -> Point {
  Point { x, y } // Field init shorthand
}
```

Don't need to repeat the variable names when they match the field names.

```rust
fn main() {
  let x = 1.0;
  let y = 2.0;
  let point = Point { x, y }; // Field init shorthand
  println!("Point coordinates: ({}, {})", point.x, point.y);
}
```

## Building structs from other structs

New structs can be created from existing structs using the `..` syntax, which copies the remaining fields.

```rust
fn main() {
  let p1 = Point { x: 1.0, y: 2.0 };
  let p2 = Point { x: 3.0, ..p1 }; // Copies `y` from `p1`
  println!("New Point coordinates: ({}, {})", p2.x, p2.y); // (3.0, 2.0)
}
```

## Tuple structs

Tuple structs have a name but do not have named fields. They are useful for creating distinct types that are similar to tuples.

```rust
struct Color(u8, u8, u8); // RGB color
struct Point3D(f64, f64, f64); // 3D point

fn main() {
  let red = Color(255, 0, 0);
  let point = Point3D(1.0, 2.0, 3.0);

  println!("Red color: ({}, {}, {})", red.0, red.1, red.2);
  println!("3D Point coordinates: ({}, {}, {})", point.0, point.1, point.2);
}
```

## Unit-like structs

Unit-like structs do not have any fields. They are useful for creating types that carry no data but can be used for type safety or as markers.

```rust
struct Marker;

fn main() {
  let _marker = Marker; // Create an instance of the unit-like struct
  // Implementing methods or traits for unit-like structs can be useful
  println!("Marker created!");
}
```

> It is possible to store references to data in structs,
> but you need to be careful with lifetimes to avoid dangling references.

## Debugging structs

`#[derive(Debug)]` can be used to automatically implement the `Debug` trait for structs,
which allows you to print them using the `{:?}` format specifier.

```rust
#[derive(Debug)]
struct Point {
  x: f64,
  y: f64,
}

fn main() {
  let p = Point { x: 1.0, y: 2.0 };
  println!("Point: {:?}", p); // Prints: Point: Point { x: 1.0, y: 2.0 }
  // println!("Point: {p:?}")
}
```

`{:#?}` can be used for pretty-printing the struct with indentation.

```rust
println!("Point: {:#?}", p); // Prints: Point: Point {
  //  x: 1.0,
  //  y: 2.0,
  //}
}
```

### `dbg!` macro

The `dbg!()` macro is a convenient way to print debug information about variables.
It prints the value and the source location where it was called.
It prints to standard error (`stderr`)

- `dbg!` macro takes ownership of the value, and the value back,
  so it can be used in expressions and in any form of data like structs, tuples, etc.

```rust
dbg!(p); // Prints: [src/main.rs:10] p = Point { x: 1.0, y: 2.0 }
```

## Methods

Methods are functions defined within the context of a struct.

- The first parameter of a method is always `self`, which represents the instance of the struct.

Defined using `impl` block.

```rust

#[derive(Debug)]
struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn new(x: f64, y: f64) -> Self {
    Point { x, y }
  }

  fn distance(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn main() {
  let p = Point::new(3.0, 4.0);
  println!("Point: {:?}", p);
  println!("Distance from origin: {}", p.distance()); // Prints: 5.0
}
```

- `::` is used to call associated functions (like `new`) and methods on structs.
- `.` is used to access fields and methods on an instance of a struct.
- `self` is a reference to the instance of the struct, allowing access to its fields and methods.

> Rust does not have the `->` operator for methods like in some other languages.
> Instead, you use `.` to call methods on an instance of a struct.
> This is called _automatic dereferencing_.
