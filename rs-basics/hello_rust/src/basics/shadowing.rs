use std::io;

/**
 * Shdowing allows us to reuse variable names without mutability
 * Its provides flexibility to change variable types and helps keep your code concise and readable.
 */
pub fn shadowing_sample_one() {
  let a = 5;
  let a = a + 1; // shadowing the first `a`
  println!("the value of a is {}", a); // output: 6

  {
    let a = a * 2; // shadowing within a new scope
    println!("the value of a in the inner scope is {}", a); // Outputs: 12
  }

  println!("the value of a is {}", a) // Outputs: 6
}

pub fn shadowing_sample_two() {
  let spaces = "  "; // its a string
  let spaces = spaces.len(); // now spaces is a number
  println!("the length of spaces is {}", spaces) // output 3
}

pub fn shadowing_vs_mutability() {
  // Using mutability
  let mut count = 1;
  println!("initial count: {}", count);

  count = 2; // mutating the variable
  println!("count using mutability: {}", count);

  // Using shadowing
  let count = "once"; // shadowing with a different type
  let count = count.len(); // shadowing again with a different type
  println!("Count using shadowing: {}", count);
}

#[allow(dead_code)]
pub fn real_world_shadowing() {
  println!("pls enter your age: ");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let input = input.trim(); // shadowing to remove whitespace
  let input: u32 = input.parse().expect("pls enter a number!");

  println!("yout age is : {}", input);
}
