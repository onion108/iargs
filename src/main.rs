use std::env::args;

fn main() {
    for (i, arg) in args().enumerate() {
        println!("argv[{i}] = {arg:?}");
    }
}

