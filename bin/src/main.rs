use std::{env, fs};

use frontend::parser::Parser;
use tree_walk::Interpreter;

fn main() {
    let filename = env::args().last().expect("no input file");

    if let Ok(code) = fs::read_to_string(filename.clone()) {
        let mut parser = Parser::new(&code).unwrap();
        let mut interpreter = Interpreter::new();

        interpreter.add_function("print", 1, |x| {
            println!("{}", x.get(0).unwrap());
            0.0
        });

        interpreter.add_function("input", 0, |_| {
            let mut num = String::new();
            std::io::stdin().read_line(&mut num).unwrap();
            num.trim().parse().unwrap()
        });

        let stmts = parser.parse().unwrap();

        if !parser.token_stream.errors.is_empty() {
            for error in parser.token_stream.errors {
                println!("{:#?}", error)
            }
        } else {
            for stmt in stmts {
                match interpreter.eval(stmt) {
                    Ok(()) => (),
                    Err(error) => println!("{:#?}", error),
                }
            }
        }
    } else {
        println!("cannot find file '{filename}'")
    }
}
