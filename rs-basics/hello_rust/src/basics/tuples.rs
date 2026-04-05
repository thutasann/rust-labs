pub fn tuple_sample_one() {
  let tup = (500, 6.4, 1);

  // Destructuring a tuple
  let (x, y, z) = tup;

  println!("x: {}", x);
  println!("y: {}", y);
  println!("z: {}", z);

  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;

  println!("five_hundred: {}", five_hundred);
  println!("six_point_four: {}", six_point_four);
  println!("one: {}", one);
}
