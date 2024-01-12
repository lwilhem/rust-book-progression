fn main() {
  let mut s1 = String::from("Hello World !");

  // We are passing a reference to s1 as an argument
  let len = calculate_length(&s1);

  change(&mut s1);

  println!("{}", s1);

  // Because s1 ownership didn't change, we can still call i within this scope
  println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
  // s is a reference to a String
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Notes: Just like variables, references are immutables by default.
// so we have to specify the reference as mutable, just like variabels
fn change(some_string: &mut String) {
  some_string.push_str(", OJFRPOEOKR");
}
