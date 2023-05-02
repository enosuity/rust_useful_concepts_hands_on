
fn parse_csv<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
  src.lines()
     .map(|line| {
        line.map(|line|{
          line.split(',')
              .map(|entry| String::from(entry.trim()))
              .collect()
       })
     })
     .collect()
}

pub fn demo() {
  println!("==== impl Trait can be used in two locations ===> as an argument type & as a return type ====");

  let arr = [12,35,48,96];

  let a = arr.map(|x| {
    x.to_string()
  });

  
}