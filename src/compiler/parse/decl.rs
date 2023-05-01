use smallvec::SmallVec;
use xjbutil::either::Either;
use crate::compiler::SyntaxError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::parse::cst::{ConstDecl, FuncDecl};
use crate::compiler::parse::expect_n_consume;
use crate::compiler::parse::expr::parse_expr;
use crate::io_ctx::Type21;
use super::stmt::parse_block_stmt;
use super::ty::parse_types;

pub fn parse_top_level_decl(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<Either<ConstDecl, FuncDecl>, SyntaxError> {
    let cur_token = &tokens[*cursor];
    match cur_token.data {
        TokenData::SymLBracket
        | TokenData::KwdVoid
        | TokenData::KwdInt
        | TokenData::KwdFloat => Ok(Either::Right(parse_func_decl(tokens, cursor)?)),
        TokenData::KwdConst => Ok(Either::Left(parse_const_decl(tokens, cursor)?)),
        _ => Err(SyntaxError::new(cur_token.line))
    }
}

pub fn parse_func_decl(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<FuncDecl, SyntaxError> {
    let ret_types = parse_types(tokens, cursor)?;

    let cur_token = &tokens[*cursor];
    let TokenData::Ident(name) = &cur_token.data else {
        return Err(SyntaxError::new(cur_token.line));
    };

    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymLParen, cursor)?;

    let mut params: SmallVec<[(Type21, String); 2]> = SmallVec::new();
    loop {
        let cur_token = &tokens[*cursor];
        match cur_token.data {
            TokenData::SymRParen => {
                *cursor += 1;
                break;
            },
            TokenData::KwdInt | TokenData::KwdFloat => {
                let TokenData::Ident(name) = &tokens[*cursor + 1].data else {
                    return Err(SyntaxError::new(cur_token.line));
                };

                *cursor += 2;
                params.push((Type21::from_token(cur_token), name.to_string()));
            },
            _ => return Err(SyntaxError::new(cur_token.line))
        }

        let cur_token = &tokens[*cursor];
        if cur_token.data == TokenData::SymComma {
            *cursor += 1;
        } else if cur_token.data != TokenData::SymRParen {
            return Err(SyntaxError::new(cur_token.line));
        }
    }

    let cur_token = &tokens[*cursor];
    let body = match cur_token.data {
        TokenData::SymSemi => {
            *cursor += 1;
            None
        },
        TokenData::SymLBrace => {
            Some(parse_block_stmt(tokens, cursor)?)
        },
        _ => return Err(SyntaxError::new(cur_token.line))
    };

    Ok(FuncDecl {
        name: name.to_string(),
        ty: ret_types,
        params,
        body
    })
}

pub fn parse_const_decl(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<ConstDecl, SyntaxError> {
    *cursor += 1;

    let cur_token = &tokens[*cursor];
    let TokenData::Ident(name) = &cur_token.data else {
        return Err(SyntaxError::new(cur_token.line));
    };

    *cursor += 1;
    expect_n_consume(tokens, TokenData::OpAssign, cursor)?;

    let value = parse_expr(tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;

    Ok(ConstDecl {
        name: name.to_string(),
        value
    })
}
