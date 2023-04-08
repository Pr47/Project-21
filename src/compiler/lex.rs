use crate::compiler::SyntaxError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    Ident(String),
    LitInt(i32),
    LitFloat(f32),

    // Keywords
    KwdConst,
    KwdInt,
    KwdFloat,
    KwdVar,
    KwdVoid,
    KwdReturn,
    KwdIf,
    KwdElse,
    KwdWhile,
    KwdFor,
    KwdBreak,
    KwdContinue,
    KwdTrue,
    KwdFalse,

    // Operators
    OpAssign,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpAnd,
    OpOr,
    OpNot,
    OpLt,
    OpLe,
    OpGt,
    OpGe,
    OpEq,
    OpNe,

    // Symbols
    SymSemi,
    SymComma,
    SymLParen,
    SymRParen,
    SymLBrace,
    SymRBrace,
    SymLBracket,
    SymRBracket,

    // End of Input
    EOI
}

#[derive(Debug, Clone)]
pub struct Token {
    pub data: TokenData,
    pub line: usize
}

impl Token {
    pub fn new(data: TokenData, line: usize) -> Self {
        Self { data, line }
    }

    pub fn lit_int(value: i32, line: usize) -> Self {
        Self::new(TokenData::LitInt(value), line)
    }

    pub fn lit_float(value: f32, line: usize) -> Self {
        Self::new(TokenData::LitFloat(value), line)
    }

    pub fn ident(value: String, line: usize) -> Self {
        Self::new(TokenData::Ident(value), line)
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut input = input.chars().collect::<Vec<char>>();
    input.push('\0');

    let mut idx = 0;
    loop {
        let current_char = input[idx];
        match current_char {
            '\0' => break,
            '#' => {
                while input[idx] != '\n' && input[idx] != '\0' {
                    idx += 1;
                }
            },
            ' ' | '\t' | '\r' => idx += 1,
            '\n' => {
                idx += 1;
                line += 1;
            },
            '0'..='9' => lex_number(&mut tokens, &mut idx, &mut input, line)?,
            'a'..='z' | 'A'..='Z' | '_' => lex_kwd_or_ident(&mut tokens, &mut idx, &mut input, line),
            '+' => {
                idx += 1;
                tokens.push(Token::new(TokenData::OpAdd, line));
            },
            '-' => {
                idx += 1;
                tokens.push(Token::new(TokenData::OpSub, line));
            },
            '*' => {
                idx += 1;
                tokens.push(Token::new(TokenData::OpMul, line));
            },
            '/' => {
                idx += 1;
                tokens.push(Token::new(TokenData::OpDiv, line));
            },
            '%' => {
                idx += 1;
                tokens.push(Token::new(TokenData::OpMod, line));
            },
            '&' => {
                idx += 1;
                if input[idx] == '&' {
                    idx += 1;
                }
                tokens.push(Token::new(TokenData::OpAnd, line));
            },
            '|' => {
                idx += 1;
                if input[idx] == '|' {
                    idx += 1;
                }
                tokens.push(Token::new(TokenData::OpOr, line));
            },
            '!' => {
                idx += 1;
                if input[idx] == '=' {
                    idx += 1;
                    tokens.push(Token::new(TokenData::OpNe, line));
                } else {
                    tokens.push(Token::new(TokenData::OpNot, line));
                }
            },
            '<' => {
                idx += 1;
                if input[idx] == '=' {
                    idx += 1;
                    tokens.push(Token::new(TokenData::OpLe, line));
                } else {
                    tokens.push(Token::new(TokenData::OpLt, line));
                }
            },
            '>' => {
                idx += 1;
                if input[idx] == '=' {
                    idx += 1;
                    tokens.push(Token::new(TokenData::OpGe, line));
                } else {
                    tokens.push(Token::new(TokenData::OpGt, line));
                }
            },
            '=' => {
                idx += 1;
                if input[idx] == '=' {
                    idx += 1;
                    tokens.push(Token::new(TokenData::OpEq, line));
                } else {
                    tokens.push(Token::new(TokenData::OpAssign, line));
                }
            },
            ';' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymSemi, line));
            },
            ',' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymComma, line));
            },
            '(' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymLParen, line));
            },
            ')' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymRParen, line));
            },
            '{' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymLBrace, line));
            },
            '}' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymRBrace, line));
            },
            '[' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymLBracket, line));
            },
            ']' => {
                idx += 1;
                tokens.push(Token::new(TokenData::SymRBracket, line));
            },
            _ => return Err(SyntaxError::new(line))
        }
    }

    tokens.push(Token::new(TokenData::EOI, line));
    Ok(tokens)
}

pub fn lex_number(
    tokens: &mut Vec<Token>,
    idx: &mut usize,
    input: &mut Vec<char>,
    line: usize
) -> Result<(), SyntaxError> {
    let mut value = String::new();
    let mut is_float = false;
    loop {
        let current_char = input[*idx];
        match current_char {
            '0'..='9' => {
                value.push(current_char);
                *idx += 1;
            },
            '.' => {
                value.push(current_char);
                *idx += 1;
                is_float = true;
            },
            _ => break
        }
    }

    if is_float {
        let value = value.parse::<f32>().map_err(|_| SyntaxError::new(line))?;
        tokens.push(Token::lit_float(value, line));
    } else {
        let value = value.parse::<i32>().map_err(|_| SyntaxError::new(line))?;
        tokens.push(Token::lit_int(value, line));
    }

    Ok(())
}

pub fn lex_kwd_or_ident(
    tokens: &mut Vec<Token>,
    idx: &mut usize,
    input: &mut Vec<char>,
    line: usize
) {
    let mut value = String::new();
    loop {
        let current_char = input[*idx];
        match current_char {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '$' => {
                value.push(current_char);
                *idx += 1;
            },
            _ => break
        }
    }

    match value.as_str() {
        "const" => tokens.push(Token::new(TokenData::KwdConst, line)),
        "int" => tokens.push(Token::new(TokenData::KwdInt, line)),
        "float" => tokens.push(Token::new(TokenData::KwdFloat, line)),
        "var" => tokens.push(Token::new(TokenData::KwdVar, line)),
        "void" => tokens.push(Token::new(TokenData::KwdVoid, line)),
        "return" => tokens.push(Token::new(TokenData::KwdReturn, line)),
        "if" => tokens.push(Token::new(TokenData::KwdIf, line)),
        "else" => tokens.push(Token::new(TokenData::KwdElse, line)),
        "while" => tokens.push(Token::new(TokenData::KwdWhile, line)),
        "for" => tokens.push(Token::new(TokenData::KwdFor, line)),
        "break" => tokens.push(Token::new(TokenData::KwdBreak, line)),
        "continue" => tokens.push(Token::new(TokenData::KwdContinue, line)),
        "true" => tokens.push(Token::new(TokenData::KwdTrue, line)),
        "false" => tokens.push(Token::new(TokenData::KwdFalse, line)),
        _ => tokens.push(Token::ident(value, line))
    }
}
