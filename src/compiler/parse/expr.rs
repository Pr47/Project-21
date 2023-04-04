use crate::compiler::CompileError;
use crate::compiler::lex::Token;
use crate::compiler::visit::SyntaxVisitor;

pub fn parse_expr<SV>(
    _sv: &mut SV,
    _tokens: &[Token],
    _cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    todo!()
}
