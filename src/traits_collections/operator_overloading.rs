
use std::ops::{Add, Sub, Drop};

#[derive(Debug)]
struct Point {
  x: i8,
  y: i8,
}

impl Add<i8> for Point {
  type Output = Self;

  fn add(self, other: i8) -> Self {
    Point { 
      x: self.x + other,
      y: self.y + other,
    }
  }    
}

impl Sub for Point {
  type Output = Self;
  fn sub(self, other: Self) ->  Self {
    Point { 
      x: self.x - other.x,
      y: self.y - other.y,
    }      
  }
}

pub fn overload_demo() {
  println!("========== Operator Overloading ======== ");

  let name = "Upen".to_string() + "der";

  let point1 = Point {
    x: 12,
    y: 35,
  };

  let point2 = Point {
    x: 31,
    y: 45,
  };

  let res = point1 + 4;

  println!("res  => {:?}",  res); 
}

// ========================= Drop trait=====================

struct Emp {
  name: &'static str,
}

impl Drop for Emp {
    fn drop(&mut self) {
        println!("{} is dropping.", self.name);
    }
}

pub fn demo() {
  let a = Emp { name: "A" };
  {
    let _b = Emp { name: "B "};
  }
  println!("B block exiting....");
  // drop(a);
  println!("Next line ....");
}



