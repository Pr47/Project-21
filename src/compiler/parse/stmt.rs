use crate::compiler::CompileError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::visit::SyntaxVisitor;

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
        | TokenData::KwdFloat => parse_decl_stmt(sv, tokens, cursor),
        TokenData::KwdIf => parse_if_stmt(sv, tokens, cursor),
        TokenData::KwdWhile => parse_while_stmt(sv, tokens, cursor),
        TokenData::KwdReturn => parse_return_stmt(sv, tokens, cursor),
        TokenData::KwdBreak => parse_break_stmt(sv, tokens, cursor),
        TokenData::KwdContinue => parse_continue_stmt(sv, tokens, cursor),
        TokenData::SymLBrace => parse_block_stmt(sv, tokens, cursor),
        _ => parse_expr_stmt(sv, tokens, cursor)
    }
}
