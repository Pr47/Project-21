mod dummy_visitor;

use crate::compiler::lex::tokenize;
use crate::compiler::parse::parse;
use crate::compiler::parse::test::dummy_visitor::DummyVisitor;

#[test]
fn test_simple() {
    let tokens = tokenize("void main() {}").unwrap();
    let mut sv = DummyVisitor();
    parse(&mut sv, &tokens).unwrap();
}

#[test]
fn test_interp() {
    let tokens = tokenize(include_str!("interp.bis")).unwrap();
    let mut sv = DummyVisitor();
    parse(&mut sv, &tokens).unwrap();
}

#[test]
fn test_spacium() {
    let tokens = tokenize(include_str!("spacium.bis")).unwrap();
    let mut sv = DummyVisitor();
    parse(&mut sv, &tokens).unwrap();
}
