pub fn array_sample_one() {
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
  let index = 3;

  println!("first: {}", first);
  println!("second: {}", second);

  match a.get(index) {
    Some(element) => println!("the value of the element index: {index} is : {element}"),
    None => println!("Invalid index!"),
  }
}
