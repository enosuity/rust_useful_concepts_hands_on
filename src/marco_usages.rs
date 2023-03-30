#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

struct Point {
  x: i32,
  y: i32,
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

