use async_std::task::{self, block_on}; 
use std::io::{self, Write};
use std::fs::File;
use std::time::Duration;

use std::cell::Cell;

// by doing so, Page will be partial completed. To deploy on production with partial design, It will look like odd.

#[derive(Debug)]
struct Emp<'a> {  // Trying with Cell in rust - immutation obj with some mutation fields.
  name: &'a str,
  salary: Cell<i32>,
  category: &'a str,
}

fn get_data<'a>(input: &'a str) -> io::Result<String> {
  println!("Enter {input}");

  let mut my_str = String::new();
  io::stdin().read_line(&mut my_str)?;
  Ok(my_str)
}

async fn write_info<'a>(emp: &Emp<'a>) -> io::Result<()> {
  let mut file = match File::options()
                                      .read(true)
                                      .write(true)
                                      .open("hello.txt") {
                                        Ok(file) => file,
                                        Err(error) => File::create("hello.txt")?
                                      };

  file.write_all(format!("Name: {} \n ", emp.name).as_bytes())?;
  file.write_all(format!("Salary: {:?} \n ", emp.salary).as_bytes())?;
  file.write_all(format!("Category: {} \n ", emp.category).as_bytes())?;  

  Ok(())
}



pub fn demo() {


  
  let name = match get_data("Name") {
    Ok(name) => name,
    Err(error) => panic!("error: {error}")
  };

  let sal = match get_data("salary") {
    Ok(sal) => sal,
    Err(error) => panic!("error: {error}"),
  };
  let salary = Cell::new(sal.trim().parse().unwrap());

  let category = match get_data("category") {
    Ok(category) => category,
    Err(error) => panic!("error: {error}"),
  };

  let emp = Emp { 
    salary,
    category: category.trim(),
    name: name.trim(),
   };
  println!("emp ====> {:?}", emp);

  task::block_on(write_info(&emp));  
}

async fn hold_on() {
  task::sleep(Duration::from_secs(1)).await;
  match task::try_current() {
    Some(t) => println!("The name of this task is == {:?}", t.name()),
    None    => println!("Not inside a task!"),
  }
}

pub fn use_me() {
  println!(" =========  Starting =========");
  for i in (1..10) {
    block_on(hold_on());    
    println!("i ==> {i}")
  }

  let handle = task::spawn(async {
    2+2
  });

  println!("===> {:#?}", block_on(handle));

  let emp = Emp {
    name: "Narangi",
    salary: Cell::new(150),
    category: "L2"
  };

  emp.salary.set(150);
  println!("emp with ===> {:?}", emp)
}








