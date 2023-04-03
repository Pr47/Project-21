use smallvec::SmallVec;
use crate::compiler::CompileError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::visit::SyntaxVisitor;
use crate::io_ctx::Type21;
use super::expect_token;
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
        | TokenData::KwdInt
        | TokenData::KwdFloat => parse_func_decl(sv, tokens, cursor),
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
    let ret_types = parse_types(sv, tokens, cursor)?;

    let cur_token = &tokens[*cursor];
    let TokenData::Ident(name) = &cur_token.data else {
        return Err(CompileError::syntax_error(cur_token.line));
    };

    *cursor += 1;
    expect_token(sv, tokens, TokenData::SymLParen, cursor)?;

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

                params.push((Type21::from_token(cur_token), name));
            },
            _ => return Err(CompileError::syntax_error(cur_token.line))
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
            let body = todo!();
            sv.visit_func_decl(&ret_types, &name, &params, Some(body))
                .map_err(|e| CompileError::sv_error(e, cur_token.line))
        },
        _ => Err(CompileError::syntax_error(cur_token.line))
    }
}
