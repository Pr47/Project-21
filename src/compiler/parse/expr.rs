use smallvec::SmallVec;
use crate::compiler::{CompileError, SyntaxError};
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::op::BinaryOp;
use crate::compiler::parse::expect_n_consume;
use crate::compiler::visit::SyntaxVisitor;
use crate::io_ctx::Type21;

fn is_assign_op(token_data: &TokenData) -> bool {
    match token_data {
        TokenData::OpAssign
        | TokenData::OpAddAssign
        | TokenData::OpSubAssign
        | TokenData::OpMulAssign
        | TokenData::OpDivAssign
        | TokenData::OpModAssign => true,
        _ => false
    }
}

fn token_as_lit_bool(token_data: &TokenData) -> bool {
    match token_data {
        TokenData::KwdTrue => true,
        TokenData::KwdFalse => false,
        _ => unreachable!()
    }
}

pub fn parse_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let current_token = &tokens[*cursor];
    match &current_token.data {
        TokenData::SymLBracket => parse_multi_assign_expr(sv, tokens, cursor),
        TokenData::Ident(_) => {
            let next_token = &tokens[*cursor + 1];
            if is_assign_op(&next_token.data) {
                parse_single_assign_expr(sv, tokens, cursor)
            } else {
                parse_bin_expr(sv, tokens, cursor)
            }
        },
        _ => parse_bin_expr(sv, tokens, cursor)
    }
}

pub fn parse_multi_assign_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let line = tokens[*cursor].line;

    let ident_list = parse_ident_list(tokens, cursor)?;

    expect_n_consume(tokens, TokenData::OpAssign, cursor)?;

    let expr = parse_bin_expr(sv, tokens, cursor)?;

    sv.visit_assign2(&ident_list, expr)
        .map_err(|e| CompileError::sv_error(e, line))
}

pub fn parse_single_assign_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let line = tokens[*cursor].line;

    let TokenData::Ident(ident) = &tokens[*cursor].data else { unreachable!() };
    *cursor += 1;

    let assign_op = &tokens[*cursor].data;
    *cursor += 1;

    let expr = parse_bin_expr(sv, tokens, cursor)?;

    sv.visit_assign(assign_op, ident, expr)
        .map_err(|e| CompileError::sv_error(e, line))
}

pub fn parse_bin_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    parse_bin_expr_impl(sv, tokens, cursor, 0)
}

fn parse_bin_expr_impl<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize,
    min_precedence: u8
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let mut lhs = parse_unary_expr(sv, tokens, cursor)?;

    loop {
        let current_token = &tokens[*cursor];
        let Ok(op): Result<BinaryOp, ()> = (&current_token.data).try_into() else {
            return Ok(lhs);
        };
        let precedence = op.precedence();

        if precedence < min_precedence {
            break;
        }

        *cursor += 1;

        let rhs = parse_bin_expr_impl(sv, tokens, cursor, precedence + 1)?;

        lhs = sv.visit_bin_op(op, lhs, rhs)
            .map_err(|e| CompileError::sv_error(e, current_token.line))?;
    }

    Ok(lhs)
}

pub fn parse_unary_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let current_token = &tokens[*cursor];
    match current_token.data {
        TokenData::OpNot | TokenData::OpSub => {
            *cursor += 1;
            let expr = parse_unary_expr(sv, tokens, cursor)?;
            sv.visit_uop((&current_token.data).into(), expr)
                .map_err(|e| CompileError::sv_error(e, current_token.line))
        },
        _ => parse_atom_expr(sv, tokens, cursor)
    }
}

pub fn parse_atom_expr<'a, SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let current_token = &tokens[*cursor];
    match &current_token.data {
        TokenData::Ident(name) => {
            *cursor += 1;
            if let TokenData::SymLParen = &tokens[*cursor].data {
                parse_func_call(sv, tokens, cursor, name)
            } else {
                sv.visit_ident(name)
                    .map_err(|e| CompileError::sv_error(e, current_token.line))
            }
        },
        TokenData::LitInt(value) => {
            *cursor += 1;
            Ok(sv.visit_lit_int(*value))
        },
        TokenData::LitFloat(value) => {
            *cursor += 1;
            Ok(sv.visit_lit_float(*value))
        },
        TokenData::KwdTrue | TokenData::KwdFalse => {
            let b = token_as_lit_bool(&current_token.data);
            *cursor += 1;
            Ok(sv.visit_lit_bool(b))
        },
        TokenData::KwdInt | TokenData::KwdFloat => {
            *cursor += 1;
            expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
            let expr = parse_unary_expr(sv, tokens, cursor)?;
            expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
            sv.visit_type_cast(Type21::from_token(&current_token), expr)
                .map_err(|e| CompileError::sv_error(e, current_token.line))
        },
        TokenData::SymLParen => {
            *cursor += 1;
            let expr = parse_expr(sv, tokens, cursor)?;
            expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
            Ok(expr)
        },
        _ => Err(CompileError::syntax_error(current_token.line))
    }
}

fn parse_func_call<'a, SV>(
    sv: &mut SV,
    tokens: &'a [Token],
    cursor: &mut usize,
    name: &str
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let line = tokens[*cursor].line;
    *cursor += 1;

    let mut args: SmallVec<[SV::ExprResult; 4]> = SmallVec::new();
    loop {
        let current_token = &tokens[*cursor];
        match &current_token.data {
            TokenData::SymRParen => {
                *cursor += 1;
                break;
            },
            _ => {
                let expr = parse_expr(sv, tokens, cursor)?;
                args.push(expr);
            }
        }
        // comma
        if let TokenData::SymComma = &tokens[*cursor].data {
            *cursor += 1;
        } else if let TokenData::SymRParen = &tokens[*cursor].data {
            *cursor += 1;
            break;
        } else {
            return Err(CompileError::syntax_error(current_token.line));
        }
    }

    sv.visit_call(name, &args)
        .map_err(|e| CompileError::sv_error(e, line))
}

fn parse_ident_list<'a>(
    tokens: &'a [Token],
    cursor: &mut usize
) -> Result<SmallVec<[&'a str; 2]>, SyntaxError> {
    *cursor += 1;
    let mut idents = SmallVec::new();
    loop {
        let current_token = &tokens[*cursor];
        match &current_token.data {
            TokenData::Ident(name) => {
                idents.push(name.as_str());
                *cursor += 1;
            },
            TokenData::SymRBracket => {
                *cursor += 1;
                break;
            },
            _ => return Err(SyntaxError::new(current_token.line))
        }
    }
    *cursor += 1;

    Ok(idents)
}
