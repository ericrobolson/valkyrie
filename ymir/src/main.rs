fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Paren {
    Left,
    Right,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Number {
    I32(i32),
    U32(u32),
    F32(f32),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Tokens {
    Mutable,
    Number(Number),
    Paren(Paren),
    Comment(String),
    String(String),
    Identifier(String),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum LexErr {}

pub fn add_spaces(program: String) -> String {
    // TODO: test
    program.replace("(", " ( ").replace(")", " ) ")
}

pub fn lex(program: String) -> Result<Vec<Tokens>, LexErr> {
    let program = add_spaces(program);

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_print_returns_expected() {
        let program = "(print (+ 1 2))".to_string();
        let result = lex(program);
        assert_eq!(true, result.is_ok());

        let expected = vec![
            Tokens::Paren(Paren::Left),
            Tokens::Identifier("print".into()),
            Tokens::Paren(Paren::Left),
            Tokens::Identifier("+".into()),
            Tokens::Number(Number::I32(1)),
            Tokens::Number(Number::I32(2)),
            Tokens::Paren(Paren::Right),
            Tokens::Paren(Paren::Right),
        ];
        let actual = result.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_mutable_assign_returns_expected() {
        let program = "(assign (mut:duh) (+ 1 2))".to_string();
        let result = lex(program);
        assert_eq!(true, result.is_ok());

        let expected = vec![
            Tokens::Paren(Paren::Left),
            Tokens::Identifier("print".into()),
            Tokens::Paren(Paren::Left),
            Tokens::Identifier("+".into()),
            Tokens::Number(Number::I32(1)),
            Tokens::Number(Number::I32(2)),
            Tokens::Paren(Paren::Right),
            Tokens::Paren(Paren::Right),
        ];
        let actual = result.unwrap();

        assert_eq!(expected, actual);
    }
}
