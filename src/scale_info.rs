use parity_scale_codec::{Encode, Decode, Compact, HasCompact};
use parity_scale_codec_derive::{Encode, Decode};
use bounded_vec::BoundedVec;
use sp_std::collections::btree_map::BTreeMap;
use clap::Parser;

#[derive(Debug, PartialEq, Encode, Decode)]
enum MyDemo {
    #[codec(index = 15)]
    A, 
    B(u32, u64),
    C {
        a: u32,
        b: u64,
    },
}
#[derive(Debug, PartialEq, Encode, Decode)]
struct Student {
    name: String,
    salary: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MyTest<T: HasCompact> {
    #[codec(encoded_as = "<T as HasCompact>::Type")]
    // #[codec(compact)]
    num: T,
}

pub fn demo() {
  let st = Student {
        name: "Eno Suity".to_string(),
        salary: 2345,
    };

    let st_encode = st.encode();
    println!("st encode ==============> {:?}", st_encode);

    let decoded: Student = Decode::decode(&mut &st_encode[..]).unwrap();
    println!("decoded ======> {:?}", decoded);

    let mut a: BTreeMap<&str, &str> =  BTreeMap::new();
    a.insert("hello", "HELLO");
    a.insert("anil", "ANIL");
    a.insert("pranjal", "PRANJAL");
    a.insert("23", "230");
    println!("a ====> {:?}", a);

    let st = a.get("pranjal");
    let at: &str =  match &st {
        Some(&n) => n,
        None => ""
    };

    println!("at =====> {:?}", at);    
    let data: BoundedVec<_, 2, 8> = BoundedVec::from_vec(vec![1u8, 15]).unwrap();
    let data: BoundedVec<_, 2, 8> = vec![2u8, 8].try_into().unwrap();
    println!("=============== Starting ==============");
    // assert_eq!(data.to_vec(), vec![2, 8]);
    println!("data slice ===> {:?}", data.as_slice());
    assert_eq!(*data.last(), 8);
    println!("=============== ended ==============");
}