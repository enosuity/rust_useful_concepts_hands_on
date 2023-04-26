#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}


#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer_fn() -> u32 { 500 }".parse().unwrap()
}


#[proc_macro_attribute]
pub fn log_entry_and_exit(args: TokenStream, input: TokenStream) -> TokenStream {
    let x = format!(r#"
        fn dummy() {{
            println!("entering");
            println!("args tokens: {{}}", {argsa});
            println!("input tokens: {{}}", {inputa});
            println!("exiting");
        }}
        fn my_goal() {{
            println!("======= Entering Goal ====> ");
            println!("Args tokens: {{}}", {argsa});
            println!("Input Tokens : {{}}", {inputa});
            println!("=========== Existing ============");
        }}
    "#,
            argsa = args.into_iter().count(),
            inputa = input.into_iter().count(),

            // my_args = args.into_iter().count(),
            // my_inputs = input.into_iter().count(),
    );

    x.parse().expect("Generated invalid tokens")
}


