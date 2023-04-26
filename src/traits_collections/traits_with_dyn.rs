use rand::prelude::*;

trait Animal {
  fn noise(&self) -> &'static str;    
}

struct Sheep {}
struct Cow {}

impl Animal for Sheep {
  fn noise(&self) -> &'static str {
    "baaaaaaah!"
  }
}

impl Animal for Cow {
  fn noise(&self) -> &'static str {
    "mooooooo!"
  }
}

fn anyone(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep{})
  } else {
    Box::new(Cow{})
  }
}

pub fn demo() {
  let mut rng = rand::thread_rng();

  let random_number: f64 = rng.gen();

  println!("random number ====> {}", random_number);

  let animal = anyone(random_number);

  println!("You've randomly chosen an animal, and it says {}", animal.noise());
}