use std::{thread, time::Duration};

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShirtColor>,
}

impl Inventory {
  fn giveaway(&self, user_prefer: Option<ShirtColor>) -> ShirtColor {
    user_prefer.unwrap_or_else (|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut red_no = 0;
    let mut blue_no = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => red_no += 1,
        ShirtColor::Blue => blue_no += 1,
      }
    }

    if red_no > blue_no {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

pub fn demo() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };

  let user_prefer1 = Some(ShirtColor::Red);

  let giveaway1 = store.giveaway(user_prefer1);
  println!("The user with preference {:?} gets {:?}", user_prefer1, giveaway1);

  let user_pref2 = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!(
      "The user with preference {:?} gets {:?}",
      user_pref2, giveaway2
  );
}
#[derive(Debug, Clone)]
struct Emp<'a> {
  name: &'a str,
  salary: u32,
}



pub fn annotations() {
  let mut list = [
    Emp{name: "Anuj", salary: 234},
    Emp{name: "Goldy", salary: 254},
    Emp{name: "Niku", salary: 645},
    Emp{name: "kila", salary: 123},
  ];

  let mut x = 0 ;

  list.sort_by_key(|r| { 
    x += 1;
    r.salary 
  });

  println!("list ===> {:?}", list);
}