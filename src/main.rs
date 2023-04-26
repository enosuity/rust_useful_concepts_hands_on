// mod opt_struct;
// mod clapping;
// mod scale_info;
// mod use_of_strip_prefix;
// mod system_timing;
// mod closure_info;
// mod marco_usages;
// mod use_async_task;
// mod advanced_traits;
// mod traits_collections;
mod traits_collections;
use traits_collections::run;

struct Emp {
    Name: String,
    Sno: String,
    Salary: usize
}

fn demo(arr: &mut[i16; 5]) {
    for i in 0..5 {
        if arr[i] <= 0 { arr[i] *= 3 }
    }
} 

fn tero(mut arr: [i16; 5] ) -> [i16; 5] {
    for i in 0..5 {
        if arr[i] <= 0 { arr[i] *= 2 }
    }
    arr 
}

fn main() {

    run();

    // use_async_task::demo();
    // use_async_task::use_me();
    // closure_info::annotations()

    // opt_struct::demo();
    // clapping::testing();
    // scale_info::demo();
    // use_of_strip_prefix::demo(); 
    // system_timing::demo(); 
    
    // marco_usages::use_marco();
  
    // // let msg = Message::C { x: 25, y: 56 };

    // // match msg {
    // //     Message::C { x, y } => println!("sum ==> {}", x+y),
    // //     Message::A => (),
    // //     Message::B(x) => println!("x ==> {}", x),
    // //     Message::D(a,b ,c ) => println!("Sum ==> {}", (a+b+c)),
    // // }

    // if let Message::C { x, y } = msg {
    //     println!("sum  by if let ==> {}", x+y);
    // }
    
}
