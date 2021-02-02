use std::io::Write;
use std::{io, iter::Peekable};

fn main() {
    println!("Begin YmirV0 REPL");
    println!("Press Ctrl+c to Exit");
    loop {
        print!("ymir> ");
        io::stdout().flush();

        // Do the reading
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(size) => {
                // Do the eval
                match eval(input) {
                    Ok(result) => {
                        // Do the print
                        println!("{:?}", result);
                    }
                    Err(error) => {
                        println!("ERROR: {:?}", error);
                    }
                }
            }
            Err(error) => {
                println!("ERROR: {:?}", error);
            }
        }

        // Do the loop
    }
}

pub fn eval(input: String) -> Result<String, String> {
    let tokens = match lex(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(format!("{:?}", e));
        }
    };

    Ok(format!("{:?}", tokens))
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Paren {
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    String(String),
    Bool(bool),
    Number(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    /// Left parenthesis '('
    LParen,
    /// Right parenthesis ')'
    RParen,
    /// The specifier character ':'
    Specifier,
    // A symbol specified like "'test-symbol".
    Symbol(String),

    // TODO: implement
    Comment(String),
    Literal(Literal),
    Identifier(String),
    Type(Box<Token>),
    Mutable,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LexErr {
    pub line_number: usize,
    pub character_number: usize,
    pub message: String,
}

#[derive(Clone)]
enum WorkingState {
    None,
    Symbol(String),
}

struct ParseResult {
    new_state: WorkingState,
    added_tokens: Vec<Token>,
}

fn parse_character(
    token: char,
    state: &WorkingState,
    character_number: &mut usize,
    line_number: &mut usize,
    is_final_token: bool,
) -> Result<ParseResult, LexErr> {
    let mut new_state = state.clone();

    let mut token_to_add = None;
    let mut added_tokens = vec![];
    let mut terminated_symbol = is_final_token;
    let mut is_newline = false;

    match token {
        '(' | ')' => {
            token_to_add = match token {
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                _ => None,
            };
            terminated_symbol = true;
        }
        ' ' => {
            terminated_symbol = true;
        }
        '\r' | '\n' => {
            is_newline = true;
            terminated_symbol = true;
        }
        ':' => {
            terminated_symbol = true;
            token_to_add = Some(Token::Specifier);
        }
        '\'' => match state {
            WorkingState::None => {
                new_state = WorkingState::Symbol(String::new());
            }
            WorkingState::Symbol(_) => {
                let line_number = *line_number;
                let character_number = *character_number;

                return Err(LexErr{
                    line_number, character_number, message: "Found a symbol while parsing a previous symbol! E.g. `('symbol-1'symbol2')` should be `('symbol1 'symbol2)`.".into()
                });
            }
        },
        _ => match state {
            WorkingState::None => {
                todo!("Unhandled token: '{:?}'", token)
            }
            WorkingState::Symbol(symbol) => {
                let mut symbol = symbol.clone();
                symbol.push(token);
                new_state = WorkingState::Symbol(symbol);
            }
        },
    }

    new_state = match &new_state {
        WorkingState::None => WorkingState::None,
        WorkingState::Symbol(symbol) => {
            if terminated_symbol {
                added_tokens.push(Token::Symbol(symbol.clone()));
                WorkingState::None
            } else {
                new_state
            }
        }
    };

    match token_to_add {
        Some(token) => {
            added_tokens.push(token);
        }
        None => {}
    }

    if is_newline {
        *line_number += 1;
        *character_number = 0;
    } else {
        *character_number += 1;
    }

    Ok(ParseResult {
        new_state,
        added_tokens,
    })
}

pub fn lex(program: String) -> Result<Vec<Token>, LexErr> {
    let mut tokens = vec![];

    let mut line_number = 0;
    let mut character_number = 0;
    let mut state = WorkingState::None;
    let program = program.chars().collect::<Vec<char>>();

    for (index, token) in program.iter().enumerate() {
        let parse_result = parse_character(
            *token,
            &state,
            &mut character_number,
            &mut line_number,
            index == program.len() - 1,
        )?;

        state = parse_result.new_state;

        for new_token in parse_result.added_tokens {
            tokens.push(new_token);
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_symbol_returns_ok() {
        let (s, expected) = ("'as-df", vec![Token::Symbol("as-df".into())]);
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = ("'asdf", vec![Token::Symbol("asdf".into())]);
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = ("'asdf    ", vec![Token::Symbol("asdf".into())]);
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = ("      'asdf   \n ", vec![Token::Symbol("asdf".into())]);
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = ("'asdf \n ", vec![Token::Symbol("asdf".into())]);
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "'asdf \n ('blah) 'loblaww",
            vec![
                Token::Symbol("asdf".into()),
                Token::LParen,
                Token::Symbol("blah".into()),
                Token::RParen,
                Token::Symbol("loblaww".into()),
            ],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "'asdf('fjda",
            vec![
                Token::Symbol("asdf".into()),
                Token::LParen,
                Token::Symbol("fjda".into()),
            ],
        );

        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_specifier_returns_expected() {
        let strings = vec![
            (
                ":(:",
                vec![Token::Specifier, Token::LParen, Token::Specifier],
            ),
            (":", vec![Token::Specifier]),
        ];
        for (s, expected) in strings {
            let result = lex(s.into());
            assert_eq!(true, result.is_ok());

            let actual = result.unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn lex_paren_returns_expected() {
        let strings = vec![
            ("(", vec![Token::LParen]),
            (")", vec![Token::RParen]),
            ("()", vec![Token::LParen, Token::RParen]),
            ("()\r\n", vec![Token::LParen, Token::RParen]),
            (
                "(()())",
                vec![
                    Token::LParen,
                    Token::LParen,
                    Token::RParen,
                    Token::LParen,
                    Token::RParen,
                    Token::RParen,
                ],
            ),
        ];
        for (s, expected) in strings {
            let result = lex(s.into());
            assert_eq!(true, result.is_ok());

            let actual = result.unwrap();
            assert_eq!(expected, actual);
        }
    }
}
