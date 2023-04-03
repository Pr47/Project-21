use crate::compiler::CompileError;
use crate::compiler::lex::Token;
use crate::compiler::visit::SyntaxVisitor;

pub fn parse_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    todo!()
}
