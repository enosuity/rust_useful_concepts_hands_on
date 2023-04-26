
mod sample;
mod measure;
mod traits_with_dyn;
mod operator_overloading;
pub mod new_samasya;
pub mod use_iterators;

// To run any above - just use demo in every module like below-
pub use use_iterators::demo;


pub fn run() {
  demo();
}

pub mod goldy {
  pub fn lelo() {
    println!("Kya leke aaye the, kya leke jaoge");
  }
}
