use std::{env, fs};

use tree_walk::Interpreter;

fn main() {
    let filename = env::args().last().expect("no input file");

    let Ok(code) = fs::read_to_string(filename.clone()) else {
        panic!("cannot find file '{filename}'");
    };

    let mut interpreter = Interpreter::default();

    interpreter.add_function("print", 1, |x| {
        println!("{}", x.first().unwrap());
        0.0
    });

    interpreter.add_function("input", 0, |_| {
        let mut num = String::new();
        std::io::stdin().read_line(&mut num).unwrap();
        num.trim().parse().unwrap()
    });

    match interpreter.eval(&code) {
        Ok(()) => (),
        Err(errors) => {
            for error in errors {
                println!("{error}");
            }
        }
    };
}
