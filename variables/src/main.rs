use std::io;

// fn main() {
//   let x = 5;
//   println!("The value of x is: {x}");
//   x = 6;
//   println!("The value of x is: {x}");
// }
// x is immutable, by default, Rust Compiler throws an error when trying to change x .
fn main() {
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");
  print_hours();
  shadowed_x();
  array_example();
}
// here, x can change because it is marked as mutable, compiler is ok

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// Always immutable, cannot change at runtime, has to be annotated
// useful to declare values at the global scope

const TUPLE: (i32, f64, &str, bool) = (1, 3.45, "UJEKDLEF", false);
// Tuples Comes in parenthesis

fn print_hours() {
  let (_x, y, z, e) = TUPLE;
  // we can use pattern matching to destructure a Tuple

  let access = TUPLE.0;
  // can also access with tuple.index syntax

  println!("{access}, {y}, {z}, {e}");

  println!("3 hours in second: {THREE_HOURS_IN_SECONDS}")
}

fn shadowed_x() {
  let y = 5;
  let y = y + 1;

  // Scope matters with variables values
  {
    // Shadowing example
    let y = y * 2;
    println!("The value of x in the inner scope is: {y}");

    let y = "  ";
    let y = y.len();

    println!("{y} : Non mutable shadowed variables can change type, mutable need same type")
  }

  println!("The value of x is: {y}");
}

fn array_example() {
  // types = [type of values; number of values]
  // [value; number of value] also works
  // Array only accept a single type of values
  let a = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index)
    .expect("Error while reading IO");

  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

  let element = a[index];
  // If index doesn't exist in array, Programm will panic and exit .
  // it's normal behavior to forbid unpredicted memory access

  println!("The value of the element at index {index} is: {element}");
}
