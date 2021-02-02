use regex::Regex;
use std::io::Write;
use std::{io, iter::Peekable};

fn main() {
    println!("Begin YmirV0 REPL");
    println!("Press Ctrl+c to Exit");
    loop {
        print!("ymir> ");
        io::stdout().flush().unwrap();

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

const MUTABLE: &'static str = "mut";
const TRUE: &'static str = "true";
const FALSE: &'static str = "false";

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Paren {
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    /// The boolean type
    Bool(bool),
    /// The integer type
    I32(i32),
    /// The string type
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    /// Left parenthesis '('
    LParen,
    /// Right parenthesis ')'
    RParen,
    /// The specifier character ':'
    Specifier,
    /// A symbol specified like "'test-symbol".
    Symbol(String),
    /// The mutable specifier 'mut'.
    Mutable,
    /// A literal type.
    Literal(Literal),
    /// Identifier
    Identifier(String),
    // Comment. Starts with '/*' and ends with  '*/'
    Comment(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct LexErr {
    pub line_number: usize,
    pub character_number: usize,
    pub message: String,
}

#[derive(Clone, Debug)]
enum WorkingState {
    None,
    Symbol(String),
    Identifier(String),
    String(String),
    Comment(String),
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
    // TODO: test all error handling.

    let mut new_state = state.clone();

    let mut token_to_add = None;
    let mut added_tokens = vec![];
    let mut terminated_symbol = is_final_token;
    let mut is_newline = false;

    let add_to_string = |c: char, string_state: &WorkingState| -> Option<WorkingState> {
        match string_state {
            WorkingState::String(s) => {
                let mut s = s.clone();
                s.push(c);

                Some(WorkingState::String(s))
            }
            WorkingState::Comment(s) => {
                let mut s = s.clone();
                s.push(c);

                Some(WorkingState::Comment(s))
            }
            _ => None,
        }
    };

    match token {
        '(' | ')' => {
            //
            match add_to_string(token, state) {
                Some(s) => {
                    new_state = s;
                }
                None => {
                    token_to_add = match token {
                        '(' => Some(Token::LParen),
                        ')' => Some(Token::RParen),
                        _ => None,
                    };
                    terminated_symbol = true;
                }
            }
        }
        ' ' => {
            //
            match add_to_string(token, state) {
                Some(s) => {
                    new_state = s;
                }
                None => {
                    terminated_symbol = true;
                }
            }
        }
        '\r' | '\n' => {
            //
            match add_to_string(token, state) {
                Some(s) => {
                    new_state = s;
                }
                None => {
                    is_newline = true;
                    terminated_symbol = true;
                }
            }
        }
        ':' => {
            //
            match add_to_string(token, state) {
                Some(s) => {
                    new_state = s;
                }
                None => {
                    terminated_symbol = true;
                    token_to_add = Some(Token::Specifier);
                }
            }
        }
        '\'' => {
            //
            match add_to_string(token, state) {
                Some(s) => {
                    new_state = s;
                }
                None => match state {
                    WorkingState::Identifier(_) => {
                        let line_number = *line_number;
                        let character_number = *character_number;

                        return Err(LexErr{
                            line_number, character_number, message: "Found a symbol while parsing an identifier! `(test'symbol2')` should be `(test 'symbol2)`.".into()
                        });
                    }
                    WorkingState::None => {
                        new_state = WorkingState::Symbol(String::new());
                    }
                    WorkingState::Symbol(_) => {
                        let line_number = *line_number;
                        let character_number = *character_number;

                        return Err(LexErr{
                            line_number, character_number, message: "Found a symbol while parsing a previous symbol! `('symbol-1'symbol2')` should be `('symbol1 'symbol2)`.".into()
                        });
                    }
                    WorkingState::String(_) => {}
                    WorkingState::Comment(_) => {}
                },
            }
        }
        _ => match state {
            WorkingState::None => {
                if token == '"' {
                    new_state = WorkingState::String(String::new());
                } else {
                    let mut identifier = String::new();
                    identifier.push(token);
                    new_state = WorkingState::Identifier(identifier);
                }
            }
            WorkingState::Symbol(symbol) => {
                let mut symbol = symbol.clone();
                symbol.push(token);
                new_state = WorkingState::Symbol(symbol);
            }
            WorkingState::Identifier(identifier) => {
                let mut identifier = identifier.clone();
                identifier.push(token);

                // If it's a comment, switch it. Otherwise leave as identifier.
                if identifier.starts_with("/*") {
                    new_state = WorkingState::Comment(identifier);
                } else {
                    new_state = WorkingState::Identifier(identifier);
                }
            }
            WorkingState::String(string) => {
                // Add character to string
                match add_to_string(token, state) {
                    Some(s) => {
                        match s {
                            WorkingState::String(s) => {
                                // Only end string if it's not an escaped end quote
                                if s.ends_with("\"") {
                                    token_to_add =
                                        Some(Token::Literal(Literal::String(string.clone())));
                                    new_state = WorkingState::None;
                                } else {
                                    // set new state;
                                    new_state = WorkingState::String(s);
                                }
                            }
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
            WorkingState::Comment(string) => {
                // Add character to string
                match add_to_string(token, state) {
                    Some(s) => {
                        match s {
                            WorkingState::Comment(s) => {
                                // Only end string if it's not an escaped end quote
                                if s.ends_with("*/") {
                                    token_to_add = Some(Token::Comment(string.clone()));
                                    new_state = WorkingState::None;
                                } else {
                                    // set new state;
                                    new_state = WorkingState::Comment(s);
                                }
                            }
                            _ => {}
                        }
                    }
                    None => {}
                }
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
        WorkingState::Identifier(identifier) => {
            if terminated_symbol {
                let lowercase = identifier.to_lowercase();

                let int_result = lowercase.parse::<i32>();
                if int_result.is_ok() {
                    let result = int_result.unwrap();
                    added_tokens.push(Token::Literal(Literal::I32(result)));
                } else if lowercase.as_str() == MUTABLE {
                    added_tokens.push(Token::Mutable);
                } else if lowercase.as_str() == TRUE {
                    added_tokens.push(Token::Literal(Literal::Bool(true)));
                } else if lowercase.as_str() == FALSE {
                    added_tokens.push(Token::Literal(Literal::Bool(false)));
                } else {
                    added_tokens.push(Token::Identifier(identifier.clone()));
                }

                WorkingState::None
            } else {
                new_state
            }
        }
        WorkingState::String(s) => {
            if is_final_token {
                let line_number = *line_number;
                let character_number = *character_number;

                return Err(LexErr{
                    line_number, character_number, message: "Found an string that wasn\'t closed! `\"test string` should be `\"test string\"`.".into()
                });
            } else {
                new_state
            }
        }
        WorkingState::Comment(c) => {
            if is_final_token {
                let line_number = *line_number;
                let character_number = *character_number;

                println!("{:?}", c);

                return Err(LexErr{
                    line_number, character_number, message: "Found an comment that wasn\'t closed! `/* test comment ` should be `/* test comment */`.".into()
                });
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
    fn lex_string_returns_ok() {
        let (s, expected) = (
            "\"-2432:\n\n`'\"",
            vec![Token::Literal(Literal::String("-2432:\n\n`'".into()))],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_i32_returns_ok() {
        let (s, expected) = (
            "-2432:",
            vec![Token::Literal(Literal::I32(-2432)), Token::Specifier],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "9999932:",
            vec![Token::Literal(Literal::I32(9999932)), Token::Specifier],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "3.14:",
            vec![Token::Identifier("3.14".into()), Token::Specifier],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_bool_returns_ok() {
        let (s, expected) = (
            "true:",
            vec![Token::Literal(Literal::Bool(true)), Token::Specifier],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "tRuE:\n",
            vec![Token::Literal(Literal::Bool(true)), Token::Specifier],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "false:\n",
            vec![Token::Literal(Literal::Bool(false)), Token::Specifier],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "fALSe:\n",
            vec![Token::Literal(Literal::Bool(false)), Token::Specifier],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_mut_returns_ok() {
        let (s, expected) = ("mut:", vec![Token::Mutable, Token::Specifier]);
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "(mut:)",
            vec![
                Token::LParen,
                Token::Mutable,
                Token::Specifier,
                Token::RParen,
            ],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);

        let (s, expected) = (
            "(MuT:)",
            vec![
                Token::LParen,
                Token::Mutable,
                Token::Specifier,
                Token::RParen,
            ],
        );
        let result = lex(s.into());
        assert_eq!(true, result.is_ok());

        let actual = result.unwrap();
        assert_eq!(expected, actual);
    }

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
