use smallvec::SmallVec;
use crate::compiler::CompileError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::visit::SyntaxVisitor;
use crate::io_ctx::Type21;

use super::expect_n_consume;
use super::expr::parse_expr;

pub fn parse_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let cur_token = &tokens[*cursor];
    match cur_token.data {
        TokenData::KwdVar
        | TokenData::KwdInt
        | TokenData::KwdFloat
        | TokenData::KwdBool => parse_decl_stmt(sv, tokens, cursor),
        TokenData::KwdIf => parse_if_stmt(sv, tokens, cursor),
        TokenData::KwdWhile => parse_while_stmt(sv, tokens, cursor),
        TokenData::KwdFor => parse_for_stmt(sv, tokens, cursor),
        TokenData::KwdReturn => parse_return_stmt(sv, tokens, cursor),
        TokenData::KwdBreak => parse_break_stmt(sv, tokens, cursor),
        TokenData::KwdContinue => parse_continue_stmt(sv, tokens, cursor),
        TokenData::SymLBrace => parse_block_stmt(sv, tokens, cursor),
        _ => parse_expr_stmt(sv, tokens, cursor)
    }
}

pub fn parse_decl_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let cur_token = &tokens[*cursor];
    let ty = if let TokenData::KwdVar = cur_token.data {
        None
    } else {
        Some(Type21::from_token(cur_token))
    };

    *cursor += 1;

    let cur_token = &tokens[*cursor];
    let TokenData::Ident(name) = &cur_token.data else {
        return Err(CompileError::syntax_error(cur_token.line));
    };

    *cursor += 1;
    if let TokenData::OpAssign = &tokens[*cursor].data {
        *cursor += 1;
        let init = parse_expr(sv, tokens, cursor)?;
        expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
        let decl = sv.visit_var_decl(ty, name, Some(init))
            .map_err(|e| CompileError::sv_error(e, cur_token.line))?;
        sv.visit_decl_stmt(decl)
            .map_err(|e| CompileError::sv_error(e, cur_token.line))
    } else if let TokenData::SymSemi = tokens[*cursor].data {
        *cursor += 1;
        let decl = sv.visit_var_decl(ty, name, None)
            .map_err(|e| CompileError::sv_error(e, cur_token.line))?;
        sv.visit_decl_stmt(decl)
            .map_err(|e| CompileError::sv_error(e, cur_token.line))
    } else {
        return Err(CompileError::syntax_error(cur_token.line));
    }
}

pub fn parse_if_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
    let cond = parse_expr(sv, tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
    let then = parse_stmt(sv, tokens, cursor)?;
    let else_ = if let TokenData::KwdElse = tokens[*cursor].data {
        *cursor += 1;
        Some(parse_stmt(sv, tokens, cursor)?)
    } else {
        None
    };
    sv.visit_if_stmt(cond, then, else_)
        .map_err(|e| CompileError::sv_error(e, tokens[*cursor].line))
}

pub fn parse_while_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
    let cond = parse_expr(sv, tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
    let body = parse_stmt(sv, tokens, cursor)?;
    sv.visit_while_stmt(cond, body)
        .map_err(|e| CompileError::sv_error(e, tokens[*cursor].line))
}

pub fn parse_for_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
)-> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
    let init = if let TokenData::SymSemi = tokens[*cursor].data {
        None
    } else {
        Some(parse_expr(sv, tokens, cursor)?)
    };
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    let cond = if let TokenData::SymSemi = tokens[*cursor].data {
        None
    } else {
        Some(parse_expr(sv, tokens, cursor)?)
    };
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    let step = if let TokenData::SymRParen = tokens[*cursor].data {
        None
    } else {
        Some(parse_expr(sv, tokens, cursor)?)
    };
    expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
    let body = parse_stmt(sv, tokens, cursor)?;
    sv.visit_for_stmt(init, cond, step, body)
        .map_err(|e| CompileError::sv_error(e, tokens[*cursor].line))
}

pub fn parse_return_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;
    let ret = if let TokenData::SymSemi = tokens[*cursor].data {
        None
    } else {
        Some(parse_expr(sv, tokens, cursor)?)
    };
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    sv.visit_return_stmt(ret)
        .map_err(|e| CompileError::sv_error(e, tokens[*cursor].line))
}

pub fn parse_break_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    sv.visit_break_stmt()
        .map_err(|e| CompileError::sv_error(e, tokens[*cursor].line))
}

pub fn parse_continue_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    sv.visit_continue_stmt()
        .map_err(|e| CompileError::sv_error(e, tokens[*cursor].line))
}

pub fn parse_block_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    *cursor += 1;
    let mut stmts: SmallVec<[SV::StmtResult; 2]> = SmallVec::new();
    while tokens[*cursor].data != TokenData::SymRBrace {
        stmts.push(parse_stmt(sv, tokens, cursor)?);
    }
    expect_n_consume(tokens, TokenData::SymRBrace, cursor)?;
    sv.visit_block_stmt(&stmts)
        .map_err(|e| CompileError::sv_error(e, tokens[*cursor].line))
}

pub fn parse_expr_stmt<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::StmtResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let expr = parse_expr(sv, tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    Ok(sv.visit_expr_stmt(expr))
}
