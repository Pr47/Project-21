use smallvec::SmallVec;
use crate::compiler::SyntaxError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::op::BinaryOp;
use crate::compiler::parse::cst::{AssignExpr, AtomicExpr, BinaryExpr, Expr, FuncCall, MultiAssignExpr, TypeCast, UnaryExpr};
use crate::compiler::parse::expect_n_consume;
use crate::io_ctx::Type21;

fn token_as_lit_bool(token_data: &TokenData) -> bool {
    match token_data {
        TokenData::KwdTrue => true,
        TokenData::KwdFalse => false,
        _ => unreachable!()
    }
}

pub fn parse_expr(tokens: &[Token], cursor: &mut usize) -> Result<Expr, SyntaxError>
{
    let current_token = &tokens[*cursor];
    match &current_token.data {
        TokenData::SymLBracket => Ok(Expr::MultiAssignExpr(parse_multi_assign_expr(tokens, cursor)?)),
        TokenData::Ident(_) => {
            let next_token = &tokens[*cursor + 1];
            if next_token.data == TokenData::OpAssign {
                Ok(Expr::AssignExpr(parse_single_assign_expr(tokens, cursor)?))
            } else {
                parse_bin_expr(tokens, cursor)
            }
        },
        _ => parse_bin_expr(tokens, cursor)
    }
}

pub fn parse_multi_assign_expr(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<Box<MultiAssignExpr>, SyntaxError> {
    let ident_list = parse_ident_list(tokens, cursor)?;

    expect_n_consume(tokens, TokenData::OpAssign, cursor)?;
    let TokenData::Ident(name) = &tokens[*cursor].data else {
        return Err(SyntaxError::new(tokens[*cursor].line));
    };

    let expr = parse_func_call(tokens, cursor, name)?;

    Ok(Box::new(MultiAssignExpr {
        names: ident_list,
        value: Expr::FuncCall(expr)
    }))
}

pub fn parse_single_assign_expr(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<Box<AssignExpr>, SyntaxError> {
    let TokenData::Ident(ident) = &tokens[*cursor].data else { unreachable!() };
    *cursor += 1;

    expect_n_consume(tokens, TokenData::OpAssign, cursor)?;

    let expr = parse_bin_expr(tokens, cursor)?;

    Ok(Box::new(AssignExpr {
        name: ident.clone(),
        value: expr
    }))
}

pub fn parse_bin_expr(tokens: &[Token], cursor: &mut usize) -> Result<Expr, SyntaxError> {
    parse_bin_expr_impl(tokens, cursor, 0)
}

fn parse_bin_expr_impl(
    tokens: &[Token],
    cursor: &mut usize,
    min_precedence: u8
) -> Result<Expr, SyntaxError> {
    let mut lhs = parse_unary_expr(tokens, cursor)?;

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

        let rhs = parse_bin_expr_impl(tokens, cursor, precedence + 1)?;

        lhs = Expr::BinaryExpr(Box::new(BinaryExpr {
            op,
            lhs,
            rhs
        }));
    }

    Ok(lhs)
}

pub fn parse_unary_expr(tokens: &[Token], cursor: &mut usize) -> Result<Expr, SyntaxError> {
    let current_token = &tokens[*cursor];
    match current_token.data {
        TokenData::OpNot | TokenData::OpSub => {
            *cursor += 1;
            let expr = parse_unary_expr(tokens, cursor)?;
            Ok(Expr::UnaryExpr(Box::new(UnaryExpr {
                op: (&current_token.data).into(),
                expr
            })))
        },
        _ => parse_atom_expr(tokens, cursor)
    }
}

pub fn parse_atom_expr(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<Expr, SyntaxError> {
    let current_token = &tokens[*cursor];
    match &current_token.data {
        TokenData::Ident(name) => {
            *cursor += 1;
            if let TokenData::SymLParen = &tokens[*cursor].data {
                Ok(Expr::FuncCall(parse_func_call(tokens, cursor, name)?))
            } else {
                Ok(Expr::AtomicExpr(Box::new(AtomicExpr::Ident(name.to_string()))))
            }
        },
        TokenData::LitInt(value) => {
            *cursor += 1;
            Ok(Expr::AtomicExpr(Box::new(AtomicExpr::Integer(*value))))
        },
        TokenData::LitFloat(value) => {
            *cursor += 1;
            Ok(Expr::AtomicExpr(Box::new(AtomicExpr::Float(*value))))
        },
        TokenData::KwdTrue | TokenData::KwdFalse => {
            let b = token_as_lit_bool(&current_token.data);
            *cursor += 1;
            Ok(Expr::AtomicExpr(Box::new(AtomicExpr::Bool(b))))
        },
        TokenData::KwdInt | TokenData::KwdFloat => {
            *cursor += 1;
            expect_n_consume(tokens, TokenData::SymLParen, cursor)?;
            let expr = parse_unary_expr(tokens, cursor)?;
            expect_n_consume(tokens, TokenData::SymRParen, cursor)?;

            Ok(Expr::AtomicExpr(Box::new(AtomicExpr::TypeCast(TypeCast {
                dest: Type21::from_token(current_token),
                expr
            }))))
        },
        TokenData::SymLParen => {
            *cursor += 1;
            let expr = parse_expr(tokens, cursor)?;
            expect_n_consume(tokens, TokenData::SymRParen, cursor)?;
            Ok(expr)
        },
        _ => Err(SyntaxError::new(current_token.line))
    }
}

fn parse_func_call(
    tokens: &[Token],
    cursor: &mut usize,
    name: &str
) -> Result<Box<FuncCall>, SyntaxError> {
    *cursor += 1;

    let mut args: SmallVec<[Expr; 4]> = SmallVec::new();
    loop {
        let current_token = &tokens[*cursor];
        match &current_token.data {
            TokenData::SymRParen => {
                *cursor += 1;
                break;
            },
            _ => {
                let expr = parse_expr(tokens, cursor)?;
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
            return Err(SyntaxError::new(current_token.line));
        }
    }

    Ok(Box::new(FuncCall {
        name: name.to_string(),
        args
    }))
}

fn parse_ident_list(
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SmallVec<[String; 2]>, SyntaxError> {
    *cursor += 1;
    let mut idents = SmallVec::new();
    loop {
        let current_token = &tokens[*cursor];
        match &current_token.data {
            TokenData::Ident(name) => {
                idents.push(name.clone());
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
