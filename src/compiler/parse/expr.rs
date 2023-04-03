use smallvec::SmallVec;
use crate::compiler::CompileError;
use crate::compiler::lex::{Token, TokenData};
use crate::compiler::visit::SyntaxVisitor;
use super::{expect_token, expect_n_consume};

pub fn parse_expr<SV>(
    sv: &mut SV,
    tokens: &[Token],
    cursor: &mut usize
) -> Result<SV::ExprResult, CompileError<SV::Error>>
    where SV: SyntaxVisitor
{
    let cur_token = &tokens[*cursor];
    match cur_token.data {
        TokenData::SymLParen => {
            *cursor += 1;
            let expr = parse_expr(sv, tokens, cursor)?;
            expect_token(sv, tokens, TokenData::SymRParen, cursor)?;
            Ok(expr)
        },
        TokenData::SymLBracket => {
            *cursor += 1;
            // parse identifier list
            let mut idents: SmallVec<[&str; 2]> = SmallVec::new();
            loop {
                let cur_token = &tokens[*cursor];
                match &cur_token.data {
                    TokenData::SymRBracket => {
                        *cursor += 1;
                        break;
                    },
                    TokenData::Ident(name) => {
                        idents.push(name);
                        *cursor += 1;
                    },
                    _ => return Err(CompileError::syntax_error(cur_token.line))
                }
            }

            expect_n_consume(sv, tokens, TokenData::OpAssign, cursor)?;
            let expr = parse_expr(sv, tokens, cursor)?;
            sv.visit_assign(&idents, expr)
                .map_err(|e| CompileError::sv_error(e, cur_token.line))
        },
        _ => todo!()
    }
}
