use crate::compiler::codegen::CodegenContext;
use crate::compiler::lex::tokenize;
use crate::compiler::parse::expr::parse_expr;

#[test]
fn test_consteval() {
    let tokens = tokenize("3 + 2").unwrap();
    let mut cursor = 0;
    let expr = parse_expr(&tokens, &mut cursor).unwrap();

    let ctx = CodegenContext::new();
    dbg!(ctx.consteval_expr(&expr).unwrap());
}
