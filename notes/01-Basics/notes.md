# Rust Basics

## Variables

Default variables are immutable
Use `mut` to make a variable mutable

```rust
let x = 5; // immutable
let mut y = 10; // mutable
```

### Constants

Constants are always immutable and must have a type annotation.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Can be declared in any scope, including global scope.

Constants must be set to a constant expression,
which means they cannot be set to the result of a function call or a variable.

They can only be set to a value that can be determined at compile time.

* Convention: `SNAKE_CASE` for constants

### Shadowing

Declaring a new variable with the same name as a previous variable.
_The first variable is shadowed by the second_

Second variable overshadows the first variable until the end of the scope,
or until the second variable is shadowed by another variable.

```rust
fn main() {
  let x = 5;
  let x = x + 1; // shadowing, x is now 6
  {
    let x = x * 2; // shadowing
    println!("The value of x in the inner scope is: {}", x); // 12
  }
  println!("The value of x in the outer scope is: {}", x); // 6
}
```

We are effectively creating a new variable with the same name

The new variable can have a different type than the previous variable,
or it can be mutable while the previous variable was immutable.

#### Use Cases for Shadowing

1. Allowing you to change the type of a variable or to make an immutable variable mutable.
2. Avoiding the need for a new name for a variable when you want to change its value.
w.g., when you want to convert a string to a number, you can shadow the string variable with a new variable that has the number type.

```rust
let mut guess = String::new(); // Mutable string variable
io::stdin()
  .read_line(&mut guess)
  .expect("Failed to read line");

let guess: u32 = match guess.trim().parse().expect("Enter a number"); // Shadowing the string variable with a new variable of type u32
```

## Data Types

* Rust is a statically typed language, meaning that the type of a variable must be known at compile time.

Data types can be inferred by the compiler, so you don't always need to annotate them.

### Scalar Types

Single value types

4 main scalar types:

1. Integers
2. Floating-point numbers
3. Booleans
4. Characters

#### Integers

A number without a fractional component.

| Length | Signed | Unsigned |
| --------------- | --------------- | --------------- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

* Signed variant can store numbers from `-(2^(n-1))` to `2^(n-1) - 1`
* Unsigned variants can store numbers from `0` to `2^n - 1`

`isize` and `usize` are pointer-sized integers, meaning they are the same size as a pointer on the current architecture (32-bit or 64-bit).

Integer literals can be expressed in different formats:

| Number literals | Example      |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (`u8` only)  | b'A'        |

> Integer Overflow:
>
> When compiling for `--debug` mode, Rust will panic on integer overflow.
>
> When compiling for `--release` mode, Rust will wrap around the value, using two's complement representation.

#### Floating-point numbers

2 main types:

1. `f32` - 32-bit floating point
2. `f64` - 64-bit floating point (default)

```rust
fn main() {
  let x = 2.0; // f64

  let y: f32 = 3.0; // f32
}
```

Represented using IEEE-754 standard.

#### Boolean

1. `true`
2. `false`

#### Character

```rust
fn main() {
  let c = 'z'; // char
  let z: char = 'Z'; // char
  let heart_eyed_cat = '😻'; // Unicode character
}
```

4 bytes in size, representing a single Unicode scalar value.
Single quotes are used for characters, while double quotes are used for strings.

### Compound Types

Multiple values in a single type.

#### Tuples

General way of grouping together a number of values with different types.

Fixed length, once declared, the length cannot change.

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup; // destructuring
  println!("The value of y is: {}", y); // 6.4
}
```

To access a tuple element, use a period followed by the index of the element:

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  println!("The value of the first element is: {}", tup.0); // 500
  println!("The value of the second element is: {}", tup.1); // 6.4
  println!("The value of the third element is: {}", tup.2); // 1
}
```

#### Arrays

Every element must have the same type, and the length is fixed.

```rust
fn main() {
  let a = [1, 2, 3, 4, 5]; // array of integers
  let b: [i32; 5] = [1, 2, 3, 4, 5]; // array with type annotation
  println!("The first element of a is: {}", a[0]); // 1
}
```

Data is stored on the stack, unlike vectors which are stored on the heap.

```rust
fn main() {
  let months = [
    "January", "February", "March", "April", "May",
    "June", "July", "August", "September", "October",
    "November", "December"
  ]; // array of strings
}
```

```rust
fn main() {
  let a: [i32; 5] = [1, 2, 3, 4, 5]; // array with type annotation
  let b = [3; 5]; // array with all elements initialized to 3
  let c: [i32; 5] = [0; 5]; // array with type annotation with all elements initialized to 0
}
```

> Accessing an element outside the bounds of the array will cause a panic at runtime.

