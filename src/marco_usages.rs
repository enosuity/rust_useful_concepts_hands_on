extern crate my_macro_lib;
use std::collections::HashMap;
use my_macro_lib::*;


#[macro_export]
macro_rules! svec {
    ($($a:expr),*) => {
        {
          let mut vec = Vec::new();
          $(
            vec.push($a);
          )*
          vec.sort();
          vec
        }
    };
}

#[macro_export]
macro_rules! hash {
    ($($key:expr => $val:expr),*) => {
        {
          let mut hash_map = HashMap::new();
          $(
            hash_map.insert($key, $val);
          )*
          hash_map
        }
    };
    ($($key:expr ; $val:expr),*) => {
      {
        let mut hash_map = HashMap::new();
        $(
          hash_map.insert($key, $val);
        )*
        hash_map.len()
      }
  };
}

struct Point {
  x: i32,
  y: i32,
}

struct Emp {
  name: String,
  code: u16,
  salary: f32,
}

enum Greeting {
  Hello {id: i32},
}

make_answer!();

#[derive(AnswerFn)]
struct MyStruct;

#[log_entry_and_exit(Radhe, "world")]
fn it_will_be_destroyed() -> i32 {
  500
}

pub fn use_marco() {

  println!("================Attribute macro calling ==============");
  println!("fn answer() ===> {}", answer());
  println!("fn answer_fn() ===> {}", answer_fn());
  dummy();
  my_goal();

  
  // let st = svec!(23, 45, 67, 1, 22, 62);
  // let hash = hash!('a'=> 15, 'b'=> 150, 'c'=> 250);
  // println!("hash ==>  {:?}", hash);

  // let hass = hash!('a'; 15, 'b'; 150, 'c'; 250);
  // println!("hass ==>  {:?}", hass);

  // let mut my_hash = HashMap::new();
  // my_hash.insert('p', 152);
  // my_hash.insert('q', 15);

  
}

pub fn ignore_with_dot() {
  let greet = Greeting::Hello { id: 15 };

  match greet {
    Greeting::Hello { id: my_range @ 1..=7} => {
      println!("my_range =====> {}", my_range);
    },
    Greeting::Hello { id: 10..=15} => println!("Found in another range"),
    Greeting::Hello { id } => println!("Found some other id: {}", id),
  }


  let emp = Emp {
    name: "Goldy".to_string(),
    code: 234,
    salary: 56231.57,
  };

  match emp {
    Emp {salary, ..} => println!("In-Hand Salary => {salary}"),
  }


  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {first}, {last}");
    }
  }


}

pub fn ignore_pattern() {
  let mut a = Some(5);
  let b = Some(50);

  match (a, b) {
    (Some(_), Some(_)) => println!("Can't do it. just chill !"),
    _ => a = b
  }

  println!("a ===> {:?}", a);

  let s = Some("hello".to_string());

  if let Some(_) = s {
    println!("found String !!");
  }

  println!("{:?}", s);
}


pub fn pattern_and_match() {
  
  println!(" Hi! {} ", "Macro");

  let fav_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "3584".parse();

  if let Some(color) = fav_color {
    println!("Using your favorite color, {color}, as the background");
  }
  else if is_tuesday {
    println!("Today is Tuesday !");
  }
  else if let Ok(age) = age {
    if age >= 30 {
      println!("You are elder");
    } else {
      println!("You are younger");
    }
  } else {
    println!("Good Bye !!");
  }
  
} 

pub fn pattern_while_let() {
  let mut stack = Vec::new();
  stack.push(12);
  stack.push(20);
  stack.push(30);

  // while let Some(top) = stack.pop() {
  //   println!("top ====> {}", top);
  // }

  let a = stack.iter().enumerate();
  for (i, val) in a {
    println!("i ===> {:?}", i);
    println!("val ===> {:?}", val);
  }

  let my_tup = (23,56);
  tuple_pattern(&my_tup); 

  let x = 'x';
  match x {
    'a'..='k' => println!("getting a...k "),
    'A'..='Z' => println!("Capital latters words."),
    _ => println!("Out of the box"),
  }
}

enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(Color),
}

enum Color {
  Rgb(i32, i32, i32),
  Hsv(i32, i32, i32),
}

pub fn pattern_enum() {

  let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => 
    println!("(a, b, c) => ({r}, {g}, {b})"),
    Message::ChangeColor(Color::Hsv(h, s, v)) => 
    println!("(h, s, v) => ({h}, {s}, {v})"),
    _ => ()

  }

  // match msg {
  //     Message::Quit => println!("The Quit variant has no data to destructure."),
  //     Message::Move { x, y } => println!("Move at (x, y) => ({}, {})", x, y),
  //     Message::Write(text) => println!("Text =====> {}", text),
  //     Message::ChangeColor(r, g, b ) => println!("(r,g,b) => ({r}, {g}, {b})")
  // }

} 

pub fn pattern_struct() {
  let p = Point { x: 10, y: 15 };
  match p {
    Point { x, y: 0 } => println!("On the x axis at {} ", x),
    Point { x: 0, y } => println!("On the y axis at {} ", y),
    Point {x, y} => println!("On neither axis ({},{})", x,y),
  }
}

fn tuple_pattern(&(x, y): &(u16, u16)) {
  println!("sum ===> {}", x+y);
}

