
use structopt::StructOpt;
use strum::{VariantNames, EnumVariantNames};
// use strum_macros::EnumString;

// use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name="eno", about="This is cli app.", version = "version: 2.0.0", author = "by Anuj")]
struct Opt {
  #[structopt(short, long)]
  debug: bool,
  #[structopt(short = "v", long="velocity", default_value = "42")]
  speed: f64
}
#[derive(Debug, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum Champ {
  AnujDhiman,
  GoldyBhai,
  JackTiger,
}

pub fn testing() {
  let opt = Opt::from_args();
  println!("opt speed =====> #{:?}", opt.speed);
  println!("Champ variants ===> {:?}", Champ::VARIANTS);
}
