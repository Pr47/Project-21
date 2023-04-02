use crate::compiler::lex::Token;
use crate::compiler::visit::SyntaxVisitor;

pub fn parse<SV>(tokens: &mut Vec<Token>) -> Result<Vec<SV::DeclResult>, () /* TODO */>
    where SV: SyntaxVisitor
{
    todo!()
}
