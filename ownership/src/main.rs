fn main() {
  what_is_ownership("Hello World !");
  scope_example()
}

fn what_is_ownership(input: &str) {
  let s: String = String::from(input);
  let s1 = s.clone();
  println!("{}, {}", s, s1)
}
// After s goes out of scope, memory is automatically returned

//  let s = String::from(input);
// let s1 = s; => this expression copies pointer, len & capacity, not the string itself,
// the string is stored on the heap, the three props are stored on the stack
// when s and s1 goes out of scope, they will both try to clear the same allocation
// and the program will crash (double free error)

fn scope_example() {
  let s = String::from("hello"); // s comes into scope

  takes_ownership(s); // s's value moves into the function...
                      // ... and so is no longer valid here
                      // takes_ownership(s) error because s is no longer defined in this scope

  let x = 5; // x comes into scope

  makes_copy(x); // x would move into the function,
                 // but i32 is Copy, so it's okay to still
                 // use x afterward
  println!("{x}")
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
  // some_string comes into scope
  println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
  // some_integer comes into scope
  println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
