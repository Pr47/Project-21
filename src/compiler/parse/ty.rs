use smallvec::{SmallVec, smallvec};
use crate::io_ctx::Type21;
use crate::compiler::CompileError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::visit::SyntaxVisitor;

impl Type21 {
    pub fn from_token(token: &Token) -> Self {
        match token.data {
            TokenData::KwdInt => Type21::Int32,
            TokenData::KwdFloat => Type21::Float32,
            _ => unreachable!()
        }
    }
}

pub fn parse_types<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SmallVec<[Type21; 2]>, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let cur_token = &tokens[*cursor];
    match cur_token.data {
        TokenData::KwdInt | TokenData::KwdFloat => {
            *cursor += 1;
            Ok(smallvec![Type21::from_token(cur_token)])
        },
        TokenData::KwdVoid => {
            *cursor += 1;
            Ok(smallvec![])
        },
        TokenData::SymLBracket => {
            parse_type_list(sv, tokens, cursor)
        },
        _ => Err(CompileError::syntax_error(cur_token.line))
    }
}

pub fn parse_type_list<SV>(
    _sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SmallVec<[Type21; 2]>, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    assert_eq!(tokens[*cursor].data, TokenData::SymLBracket);
    *cursor += 1;

    let mut types = SmallVec::new();
    loop {
        let cur_token = &tokens[*cursor];
        match cur_token.data {
            TokenData::KwdInt => {
                *cursor += 1;
                types.push(Type21::Int32);
            },
            TokenData::KwdFloat => {
                *cursor += 1;
                types.push(Type21::Float32);
            },
            TokenData::SymRBracket => {
                *cursor += 1;
                break;
            },
            _ => return Err(CompileError::syntax_error(cur_token.line))
        }
    }

    Ok(types)
}
