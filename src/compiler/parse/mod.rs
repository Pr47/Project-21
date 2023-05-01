mod cst;
pub mod decl;
pub mod expr;
pub mod stmt;
pub mod ty;

use xjbutil::either::Either;
use crate::compiler::lex::Token;
use crate::compiler::parse::cst::Program;
use crate::compiler::SyntaxError;
use super::lex::TokenData;
use self::decl::parse_top_level_decl;

pub fn parse(tokens: &[Token]) -> Result<Program, SyntaxError>
{
    let mut cursor = 0;
    let mut program = Program::default();

    while cursor < tokens.len() && tokens[cursor].data != TokenData::EOI {
        let decl = parse_top_level_decl(tokens, &mut cursor)?;
        match decl {
            Either::Left(const_decl) => program.const_decl.push(const_decl),
            Either::Right(func_decl) => program.func_decl.push(func_decl)
        }
    }

    Ok(program)
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

