extern crate xshade_parser;
extern crate xshade_typecheck;

use xshade_parser::*;
use xshade_typecheck::*;

#[test]
fn it_works() {
    let source = include_str!("typetests.xs");
    let result = parse_str(source).unwrap();
    println!("{:#?}", result);
    panic!();
}
