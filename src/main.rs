use dqk_ast::{Stmt, StringAllocator};
use dqk_parser::Parser;

fn main() {
    let mut string_allocator = StringAllocator::new();

    let mut parser = Parser::new(
        "x = 0",
        string_allocator.get_path("path"),
        &mut string_allocator,
    );

    println!("{:?}", parser.parse::<Stmt>());
}
