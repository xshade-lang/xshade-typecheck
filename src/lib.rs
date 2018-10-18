extern crate xshade_parser;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod type_system;
mod typed_ast;

pub use self::typed_ast::*;
pub use self::type_system::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
