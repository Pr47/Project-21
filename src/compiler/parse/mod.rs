pub mod decl;
pub mod expr;
pub mod stmt;
pub mod ty;

use crate::compiler::lex::Token;
use crate::compiler::visit::SyntaxVisitor;
use super::CompileError;
use super::lex::TokenData;
use self::decl::parse_top_level_decl;

pub fn parse<SV>(
    sv: &mut SV,
    tokens: &[Token]
) -> Result<Vec<SV::DeclResult>, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let mut cursor = 0;
    let mut decl_results = Vec::new();
    while cursor < tokens.len() {
        let decl = parse_top_level_decl(sv, tokens, &mut cursor)?;
        decl_results.push(decl);
    }
    Ok(decl_results)
}

pub fn expect_token<SV>(
    _sv: &mut SV,
    tokens: &[Token],
    expected: TokenData,
    cursor: &mut usize
) -> Result<(), CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let cur_token = &tokens[*cursor];
    if cur_token.data == expected {
        Ok(())
    } else {
        Err(CompileError::syntax_error(cur_token.line))
    }
}

pub fn expect_n_consume<SV>(
    sv: &mut SV,
    tokens: &[Token],
    expected: TokenData,
    cursor: &mut usize
) -> Result<(), CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    expect_token(sv, tokens, expected, cursor)?;
    *cursor += 1;
    Ok(())
}
