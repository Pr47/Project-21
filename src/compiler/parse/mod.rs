pub mod decl;
pub mod expr;
pub mod stmt;
pub mod ty;

use crate::compiler::lex::Token;
use crate::compiler::SyntaxError;
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

pub fn expect_token(
    tokens: &[Token],
    expected: TokenData,
    cursor: &mut usize
) -> Result<(), SyntaxError> {
    let cur_token = &tokens[*cursor];
    if cur_token.data == expected {
        Ok(())
    } else {
        Err(SyntaxError::new(cur_token.line))
    }
}

pub fn expect_n_consume(
    tokens: &[Token],
    expected: TokenData,
    cursor: &mut usize
) -> Result<(), SyntaxError> {
    expect_token(tokens, expected, cursor)?;
    *cursor += 1;
    Ok(())
}

#[cfg(test)] mod test;
