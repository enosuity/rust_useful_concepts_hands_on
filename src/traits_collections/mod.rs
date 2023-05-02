
mod sample;
mod measure;
mod traits_with_dyn;
mod operator_overloading;
pub mod new_samasya;
mod use_iterators;
pub mod impl_trait;

// To run any above - just use demo in every module like below-
pub use impl_trait::demo;


pub fn run() {
  demo();
}

pub mod goldy {
  pub fn lelo() {
    println!("Kya leke aaye the, kya leke jaoge");
  }
}
