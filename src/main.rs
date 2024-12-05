use std::env;

mod cmd;

fn main() {
    println!("Hello, world!");
    print_usage()
}


fn print_usage(){
    let args:Vec<String>= env::args().collect();

    println!("Usage: {} [-options] class [args...] \n",args[0])
}
