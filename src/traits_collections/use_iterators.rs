
use core::iter::Iterator;

pub fn inbuilt_use_demo() {
  println!("===========How to use Iterators - already on Array and Range");

  let arr = [23, 34, 21, 90];

  println!("arr ==> {:?}", arr.into_iter());

  for (index, item) in arr.into_iter().enumerate() {
    println!("at {} item ====> {}", index, item);
  }



  let arr = 0..9;
  for (index, item) in arr.into_iter().enumerate() {
    println!("at {} item ====> {}", index, item);
  }
}

// ============= Iterator on custom Struct ==============

#[derive(Debug)]
struct  Fabo {
  current: u32,
  next: u32,
}

impl Iterator for Fabo {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
      let current = self.current;
      

      self.current = self.next;
      self.next = current + self.next;

      Some(current)
  }
    
}

fn fabo() -> Fabo {
  Fabo { current: 0, next: 1 }
}

pub fn demo() {

  println!("fabo() ==> {:?}", fabo());

  println!("The first four terms of the Fibonacci sequence are: ");
  for item in fabo().skip(4).take(5) {
      println!(" ===> {}", item);
  }

}


// =========== How to use super & Self =========

fn function() {
  println!("called `function()`");
}

mod cool {
  pub fn function() {
      println!("called `cool::function()`");
  }
}

mod my {
  fn function() {
      println!("called `my::function()`");
  }
  
  mod cool {
      pub fn function() {
          println!("called `my::cool::function()`");
      }
  }
  
  pub fn indirect_call() {
      // Let's access all the functions named `function` from this scope!
      print!("called `my::indirect_call()`, that\n> ");
      
      // The `self` keyword refers to the current module scope - in this case `my`.
      // Calling `self::function()` and calling `function()` directly both give
      // the same result, because they refer to the same function.
      self::function();
      function();
      
      // We can also use `self` to access another module inside `my`:
      self::cool::function();
      
      // The `super` keyword refers to the parent scope (outside the `my` module).
      super::function();
      
      // This will bind to the `cool::function` in the *crate* scope.
      // In this case the crate scope is the outermost scope.
      {
          use super::cool::function as root_function;
          root_function();
      }
  }
}

pub fn super_demo() {
  my::indirect_call();
}