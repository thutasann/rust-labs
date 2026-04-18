#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
  integral: u64,
  fractional: u64,
  scalar: u64,
}

impl Price {
  pub fn new(price: f64) -> Price {
    let scalar = 100000;
    let integral = price as u64;
    let fractional = ((price % 1.0) * scalar as f64) as u64;
    Price {
      scalar,
      integral,
      fractional,
    }
  }
}
