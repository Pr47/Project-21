use crate::compiler::lex::tokenize;

#[test]
fn test_lex_simple() {
    let tokens = tokenize("void main() {}").unwrap();
    dbg!(tokens);
}

#[test]
fn test_lex_interp() {
    let tokens = tokenize(include_str!("./test_resc/interp.bis")).unwrap();
    dbg!(tokens);
}

#[test]
fn test_lex_spacium() {
    let tokens = tokenize(include_str!("./test_resc/spacium.bis")).unwrap();
    dbg!(tokens);
}
