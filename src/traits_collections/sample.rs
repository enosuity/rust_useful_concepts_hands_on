

use std::ops::Add;
use std::fmt::{self, Display};

#[derive(Debug)]
struct meters(i32);
#[derive(Debug)]
struct km(i32);


impl Add for meters {
  type Output = meters;

  fn add(self, other: Self) -> Self {
    Self(self.0 + other.0)
  }
}

impl Add<km> for meters {
  type Output = meters;

  fn add(self, other: km) -> Self {
    Self(self.0 + other.0 * 1_000)
  }
    
}

pub fn add_demo() -> () {
  let len = 10;  
  println!("{}", "*".repeat(len));
  let ut = meters(1524);
  let up = meters(652);
  // println!("distance between ut and up ===> {:?}", ut.add(up));

  let dlh = km(1);
  println!("distance between ut and delhi ===> {:?}", ut.add(dlh));

  println!("{}", "=".repeat(len));

}

trait Captain {
  fn fly(&self); 
}
trait Bird {
  fn fly(&self); 
}
struct Human;

impl Human {
  fn fly(&self) {
    println!("Human flying!");
  }
}

impl Captain for Human {
  fn fly(&self) {
    println!("Captain is speaking from sky!");
  }
}

impl Bird for Human {
  fn fly(&self) {
      println!("Bird flying over roof !");
  }
}

pub fn multi_traits_demo() -> () {
  let human = Human;

  Bird::fly(&human);
  human.fly();
}

trait Animal {
  fn baby_name() -> String;    
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Tommy")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("Puppy") 
  }
}

pub fn trait_demo() -> () {
  println!("A baby dog is called a {}", Dog::baby_name());
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();

      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2 ));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2 ));
      println!("{}", "*".repeat(len + 4));
    }
}
struct Point {
  x: i32,
  y: i32,
}

impl OutlinePrint for Point { }

impl Display for Point {  
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)   
  }
}

pub fn parent_trait_demo() -> () { 
  let point = Point { 
    x: 150, 
    y: 650,
  };

  point.outline_print();  
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn wrapper_demo() {
  let wrapper = Wrapper(vec!["Hello".to_string(), "hi".to_string(), "bye".to_string()]);

  println!("wrapper ====> {}", wrapper);
}

struct Sheep {
  naked: bool,
  name: &'static str,
}

impl Sheep {
  fn shear(&mut self) {
    if self.naked {
      println!("{} is already naked...", self.name);
    } 
    else {
      println!("{} gets a haircut!", self.name);
      self.naked = true;
    }
  }    
}

trait pashu {
  fn new(name: &'static str) -> Self;

  fn name(&self) -> &'static str;
  fn noise(&self) -> &'static str;

  fn talk(&self) {
    println!("{} says {}", self.name(), self.noise());
  }
}

impl pashu for Sheep {
  fn new(name: &'static str) -> Self {
    Self { naked: false, name: name }
  }

  fn name(&self) -> &'static str {
    self.name
  }

  fn noise(&self) -> &'static str {
    if self.naked {
      "baaaaaah?"
    } else {
      "baaaaaah!"
    }
  }
}

pub fn demo() {
  let mut sheep = Sheep::new("Dolly");
  sheep.talk();
  sheep.shear();
  sheep.talk(); 
}



