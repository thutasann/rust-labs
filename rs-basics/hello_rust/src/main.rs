mod basics;

fn main() {
  println!("\n=========== Hello World ===========\n");
  basics::comments::hello_world();
  basics::comments::hello_world_2();
  // basics::functions::sample_function();

  // vars
  println!("\n=========== Vars ===========\n");
  basics::var::immutability_by_default();
  basics::var::mutable_var();

  // integers
  println!("\n=========== Scalars & Compound Data Types ===========\n");
  basics::scalar_compound_data_types::scalar_compound_data_types();
}
