pub fn immutability_by_default() {
  let y = 10;
  println!("the value of y is: {}", y);
  //   y = 20; // this line will cause a compilation error
  println!("the value of y is: {}", y)
}

pub fn mutable_var() {
  let mut y = 10;
  println!("the value of y is: {}", y);
  y = 20;
  println!("the value of y is: {}", y);
}

const SERVER_ADDRESS: &str = "192.168.1.1";

pub fn constants_sample_one() {
  println!("Server Address: {}", SERVER_ADDRESS)
}
