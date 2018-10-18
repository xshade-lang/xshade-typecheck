use xshade_parser::ast::*;
use ::type_system::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedVariableExpression {
    pub span: Span,
    pub variable_name: String,
    pub type_id: TypeId,
}

impl Spanned for TypedVariableExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
