use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub fn demo() {
  let time = SystemTime::now();
  println!("time =====>  {:?}", time);
  let ab = time
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();
  let str = format!("Unix time: {}", ab);
  let bb  = str.as_bytes();
  println!("bb  ====> {:?}", bb);      
  println!("bb to_vec ====> {:?}", bb.to_vec());
}