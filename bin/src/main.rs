use frontend::parser::Parser;

fn main() {
    let code = "let a = sosat 4len 2;fn b(x) = x * x;b(1);a = b(2);";
    let mut parser = Parser::new(code).unwrap();

    for stmt in parser.parse().unwrap() {
        println!("{:#?}", stmt)
    }

    for error in parser.token_stream.errors {
        println!("{:#?}", error)
    }
}
