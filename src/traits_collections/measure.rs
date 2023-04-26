
#[derive(Debug, PartialEq, PartialOrd)]
struct Centimeters(f64);


#[derive(Debug)]
struct Inches(f64);


impl Inches {
  fn to_centimeters(&self) -> Centimeters {
    let &Inches(inches) = self;

    Centimeters(inches as f64 * 2.54)
  }    
}


struct Miles(f64);

#[derive(PartialEq, PartialOrd)]
struct Meters(f64);

impl Miles {
    fn to_meters(&self) -> Meters {
      let &Miles(miles) = self;

      Meters(miles as f64 * 1609.34)
    }
}


pub fn demo() {
  println!("========== Centimeters & Inches ==== ");

  let meter = Centimeters(100.0);
  let foot = Inches(12.0);

  let cmp = if foot.to_centimeters() > meter {
    "bigger"
  } else {
    "smaller"
  };

  println!("One foot is {} than one meter.", cmp);

  let km = Meters(1000.0);
  let hectometer = Miles(0.0621371);

  let sdk = if hectometer.to_meters() < km  {
    "Bigger"
  } else {
    "Smaller"
  };
  println!("One km is {} than one hectometer.", sdk);
}