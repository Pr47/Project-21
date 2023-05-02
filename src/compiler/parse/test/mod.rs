use crate::compiler::lex::tokenize;
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
