#![allow(unused_imports, unused_variables)]

use crate::compiler::CompileError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::visit::SyntaxVisitor;

pub fn parse_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    /*
let current_token = &tokens[*cursor];
match current_token.data {
    TokenData::SymLBracket => parse_multi_assign_expr(sv, tokens, cursor),
    TokenData::Ident(name) => {
    },
    _ => parse_non_assign_expr()
}
*/
    todo!()
}
