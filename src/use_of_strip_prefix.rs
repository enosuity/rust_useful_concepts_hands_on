use std::str::FromStr;

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {  
  type Err = ParsePointError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {

    let (x, y) = s
      .strip_prefix("(")
      .and_then(|s| s.strip_suffix(")"))
      .and_then(|s| s.split_once(","))
      .ok_or(ParsePointError)?;

    let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
    let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;

    Ok(Point { x: x_fromstr, y: y_fromstr })
  }    
}

pub fn demo() {
  let point_1 = Point::from_str("(12,25)");

  println!("point =====> #{:?}", point_1);
}





pub fn demo_strip_prefix()  {
  // create some strings
  let str1 = "Edpre, sso";
  let str2 = "Educative";
  let str3 = "Rust";
  let str4 = "Educative is the best platform!";

  // strip some prefixes
  println!("Stripping {:?} from {} = {:?}","Ed",str1, str1.split_once(","));
  println!("Stripping {:?} from {} = {:?}","cative",str2, str2.strip_prefix("cative"));
  println!("Stripping {:?} from {} = {:?}","R",str3, str3.strip_prefix("R"));
  println!("Stripping {:?} from {} = {:?}","Educative is",str4, str4.strip_prefix("Educative is"));
}

