macro_rules! greet {
  () => {
    println!("Hello!")
  };
}

pub fn marco_sample_one() {
  greet!()
}

macro_rules! greet_2 {
  ($name:expr) => {
    println!("hello, {}", $name)
  };
}

pub fn macro_sample_two() {
  greet_2!("thuta")
}
