use smallvec::SmallVec;
use crate::compiler::CompileError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::parse::expect_n_consume;
use crate::compiler::parse::expr::parse_expr;
use crate::compiler::visit::SyntaxVisitor;
use crate::io_ctx::Type21;
use super::expect_token;
use super::stmt::parse_block_stmt;
use super::ty::parse_types;

pub fn parse_top_level_decl<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::DeclResult, CompileError<SV::Error>> 
    where SV: SyntaxVisitor
{
    let cur_token = &tokens[*cursor];
    match cur_token.data {
        TokenData::SymLBracket
        | TokenData::KwdVoid
        | TokenData::KwdInt
        | TokenData::KwdFloat => parse_func_decl(sv, tokens, cursor),
        TokenData::KwdConst => parse_const_decl(sv, tokens, cursor),
        _ => Err(CompileError::syntax_error(cur_token.line))
    }
}

pub fn parse_func_decl<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::DeclResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let ret_types = parse_types(tokens, cursor)?;

    let cur_token = &tokens[*cursor];
    let TokenData::Ident(name) = &cur_token.data else {
        return Err(CompileError::syntax_error(cur_token.line));
    };

    *cursor += 1;
    expect_token(tokens, TokenData::SymLParen, cursor)?;

    let mut params: SmallVec<[(Type21, &str); 2]> = SmallVec::new();
    loop {
        let cur_token = &tokens[*cursor];
        match cur_token.data {
            TokenData::SymRParen => {
                *cursor += 1;
                break;
            },
            TokenData::KwdInt | TokenData::KwdFloat => {
                let TokenData::Ident(name) = &tokens[*cursor + 1].data else {
                    return Err(CompileError::syntax_error(cur_token.line));
                };

                *cursor += 2;
                params.push((Type21::from_token(cur_token), name));
            },
            _ => return Err(CompileError::syntax_error(cur_token.line))
        }

        let cur_token = &tokens[*cursor];
        if cur_token.data == TokenData::SymComma {
            *cursor += 1;
        } else if cur_token.data != TokenData::SymRParen {
            return Err(CompileError::syntax_error(cur_token.line));
        }
    }

    let cur_token = &tokens[*cursor];
    match cur_token.data {
        TokenData::SymSemi => {
            *cursor += 1;
            sv.visit_func_decl(&ret_types, &name, &params, None)
                .map_err(|e| CompileError::sv_error(e, cur_token.line))
        },
        TokenData::SymLBrace => {
            let body = parse_block_stmt(sv, tokens, cursor)?;
            sv.visit_func_decl(&ret_types, &name, &params, Some(body))
                .map_err(|e| CompileError::sv_error(e, cur_token.line))
        },
        _ => Err(CompileError::syntax_error(cur_token.line))
    }
}

pub fn parse_const_decl<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::DeclResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;

    let cur_token = &tokens[*cursor];
    let TokenData::Ident(name) = &cur_token.data else {
        return Err(CompileError::syntax_error(cur_token.line));
    };

    *cursor += 1;
    expect_n_consume(tokens, TokenData::OpAssign, cursor)?;

    let expr = parse_expr(sv, tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;

    sv.visit_const_decl(name, expr)
        .map_err(|e| CompileError::sv_error(e, cur_token.line))
}
