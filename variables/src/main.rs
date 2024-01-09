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
}
// here, x can change because it is marked as mutable, compiler is ok

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// Always immutable, cannot change at runtime, has to be annotated
// useful to declare values at the global scope

fn print_hours() {
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
