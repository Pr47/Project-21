use smallvec::SmallVec;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::parse::cst::{BlockStmt, VarDecl, Stmt, IfStmt, WhileStmt, ForStmt};
use crate::compiler::parse::parse_ident_list;
use crate::compiler::SyntaxError;
use crate::io_ctx::Type21;

use super::expect_n_consume;
use super::expr::parse_expr;

pub fn parse_stmt(tokens: &[Token], cursor: &mut usize) -> Result<Stmt, SyntaxError> {
    let cur_token = &tokens[*cursor];
    match cur_token.data {
        TokenData::KwdVar
        | TokenData::KwdInt
        | TokenData::KwdFloat
        | TokenData::KwdBool => Ok(Stmt::DeclStmt(parse_decl_stmt(tokens, cursor)?)),
        TokenData::KwdIf => Ok(Stmt::IfStmt(parse_if_stmt(tokens, cursor)?)),
        TokenData::KwdWhile => Ok(Stmt::WhileStmt(parse_while_stmt(tokens, cursor)?)),
        TokenData::KwdFor => Ok(Stmt::ForStmt(parse_for_stmt(tokens, cursor)?)),
        TokenData::KwdReturn => parse_return_stmt(tokens, cursor),
        TokenData::KwdBreak => parse_break_stmt(tokens, cursor),
        TokenData::KwdContinue => parse_continue_stmt(tokens, cursor),
        TokenData::KwdYield => parse_yield_stmt(tokens, cursor),
        TokenData::SymLBrace => Ok(Stmt::BlockStmt(parse_block_stmt(tokens, cursor)?)),
        _ => parse_expr_stmt(tokens, cursor)
    }
}

pub fn parse_decl_stmt(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<Box<VarDecl>, SyntaxError> {
    let cur_token = &tokens[*cursor];
    let line = cur_token.line;
    let ty = if let TokenData::KwdVar = cur_token.data {
        None
    } else {
        Some(Type21::from_token(cur_token))
    };

    *cursor += 1;

    let cur_token = &tokens[*cursor];
    let TokenData::Ident(name) = &cur_token.data else {
        return Err(SyntaxError::new(cur_token.line));
    };

    *cursor += 1;
    let init = if let TokenData::OpAssign = &tokens[*cursor].data {
        *cursor += 1;
        let expr = parse_expr(tokens, cursor)?;
        expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
        Some(expr)
    } else if let TokenData::SymSemi = tokens[*cursor].data {
        *cursor += 1;
        None
    } else {
        return Err(SyntaxError::new(cur_token.line));
    };

    Ok(Box::new(VarDecl {
        ty,
        name: name.to_string(),
        init,

        line
    }))
}

pub fn parse_if_stmt(tokens: &[Token], cursor: &mut usize) -> Result<Box<IfStmt>, SyntaxError> {
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
    let cond = parse_expr(tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
    let then = parse_stmt(tokens, cursor)?;
    let else_ = if let TokenData::KwdElse = tokens[*cursor].data {
        *cursor += 1;
        Some(parse_stmt(tokens, cursor)?)
    } else {
        None
    };

    Ok(Box::new(IfStmt { cond, then, else_ }))
}

pub fn parse_while_stmt(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<Box<WhileStmt>, SyntaxError> {
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
    let cond = parse_expr(tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
    let body = parse_stmt(tokens, cursor)?;

    Ok(Box::new(WhileStmt { cond, body }))
}

pub fn parse_for_stmt(tokens: &[Token], cursor: &mut usize)-> Result<Box<ForStmt>, SyntaxError> {
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
    let init = if let TokenData::SymSemi = tokens[*cursor].data {
        None
    } else {
        Some(parse_expr(tokens, cursor)?)
    };
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    let cond = if let TokenData::SymSemi = tokens[*cursor].data {
        None
    } else {
        Some(parse_expr(tokens, cursor)?)
    };
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
    let step = if let TokenData::SymRParen = tokens[*cursor].data {
        None
    } else {
        Some(parse_expr(tokens, cursor)?)
    };
    expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
    let body = parse_stmt(tokens, cursor)?;

    Ok(Box::new(ForStmt { init, cond, step, body }))
}

pub fn parse_return_stmt(tokens: &[Token], cursor: &mut usize) -> Result<Stmt, SyntaxError> {
    *cursor += 1;

    match tokens[*cursor].data {
        TokenData::SymSemi => {
            *cursor += 1;
            Ok(Stmt::ReturnStmt(None))
        },
        TokenData::SymLBracket => {
            let ident_list = parse_ident_list(tokens, cursor)?;
            dbg!(&tokens[*cursor]);
            expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
            Ok(Stmt::MultiReturnStmt(ident_list))
        },
        _ => {
            let expr = parse_expr(tokens, cursor)?;
            expect_n_consume(tokens, TokenData::SymSemi, cursor)?;
            Ok(Stmt::ReturnStmt(Some(expr)))
        }
    }
}

pub fn parse_break_stmt(tokens: &[Token], cursor: &mut usize) -> Result<Stmt, SyntaxError> {
    let line = tokens[*cursor].line;
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;

    Ok(Stmt::BreakStmt(line))
}

pub fn parse_continue_stmt(tokens: &[Token], cursor: &mut usize) -> Result<Stmt, SyntaxError> {
    let line = tokens[*cursor].line;
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;

    Ok(Stmt::ContinueStmt(line))
}

pub fn parse_yield_stmt(tokens: &[Token], cursor: &mut usize) -> Result<Stmt, SyntaxError> {
    let line = tokens[*cursor].line;
    *cursor += 1;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;

    Ok(Stmt::YieldStmt(line))
}

pub fn parse_block_stmt(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<Box<BlockStmt>, SyntaxError> {
    *cursor += 1;
    let mut stmts: SmallVec<[Stmt; 4]> = SmallVec::new();
    while tokens[*cursor].data != TokenData::SymRBrace {
        stmts.push(parse_stmt(tokens, cursor)?);
    }
    expect_n_consume(tokens, TokenData::SymRBrace, cursor)?;

    Ok(Box::new(BlockStmt { stmts }))
}

pub fn parse_expr_stmt(tokens: &[Token], cursor: &mut usize) -> Result<Stmt, SyntaxError> {
    let line = tokens[*cursor].line;

    let expr = parse_expr(tokens, cursor)?;
    expect_n_consume(tokens, TokenData::SymSemi, cursor)?;

    Ok(Stmt::ExprStmt(expr, line))
}
