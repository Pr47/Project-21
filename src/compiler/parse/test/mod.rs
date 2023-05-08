use crate::compiler::lex::tokenize;
use crate::compiler::parse::expr::parse_expr;
use crate::compiler::parse::parse;

#[test]
fn test_simple() {
    let tokens = tokenize("void main() {}").unwrap();
    dbg!(parse(&tokens).unwrap());
}

#[test]
fn test_interp() {
    let tokens = tokenize(include_str!("interp.bis")).unwrap();
    dbg!(parse(&tokens).unwrap());
}

#[test]
fn test_spacium() {
    let tokens = tokenize(include_str!("spacium.bis")).unwrap();
    dbg!(parse(&tokens).unwrap());
}

#[test]
fn test_swap() {
    let tokens = tokenize(include_str!("swap.bis")).unwrap();
    dbg!(parse(&tokens).unwrap());
}

#[test]
fn test_simple_expr() {
    let tokens = tokenize("a * a + b * b > c && c > d || e == f").unwrap();
    let mut cursor = 0;
    eprintln!("{}", parse_expr(&tokens, &mut cursor).unwrap());
}
