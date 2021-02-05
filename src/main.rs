use std::{collections::HashMap, io, rc::Rc};
use std::{io::Write, num::ParseIntError};

use stack::{Stack, StackErr};
mod stack;

fn main() {
    let mut forth = ForthState::new(i16::MAX as usize);

    loop {
        print!("valkyrie> ");
        io::stdout().flush().unwrap();

        // Do the reading
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(size) => match forth.eval(input) {
                Ok(result) => match result {
                    ForthReturn::Ok => {
                        println!("OK -> STACK {:?}", forth.stack());
                    }
                    ForthReturn::Shutdown => {
                        println!("OK: Shutting down...");
                        return;
                    }
                },
                Err(error) => {
                    println!("ERROR: {:?}", error);
                }
            },
            Err(error) => {
                println!("ERROR: {:?}", error);
            }
        }

        // Do the loop
    }
}

macro_rules! set_primitive {
    ($dictionary:ident : $word:expr => $execution:expr) => {
        let action: Box<dyn Fn(&mut ForthState) -> Result<(), ForthErr>> = { Box::new($execution) };

        $dictionary
            .dictionary
            .insert($word, Rc::new(Word::Builtin(action)));
    };
}

pub enum ForthMode {
    Interpreting,
    Compiling,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ForthReturn {
    Ok,
    Shutdown,
}

/// TODO: type checking
pub enum ForthType<'a> {
    Flag(bool),
    Char(char),
    /// Signed number
    N(i32),
    /// Non-negative N
    NPlus(i32),
    /// Unsigned number
    U(u32),
    /// N | U
    UN(&'a ForthType<'a>),
}

#[derive(Debug, Clone)]
pub enum ForthErr {
    StackErr(stack::StackErr),
    DivideByZero,
    Parse(std::num::ParseIntError),
}

impl From<stack::StackErr> for ForthErr {
    fn from(se: stack::StackErr) -> Self {
        Self::StackErr(se)
    }
}

impl From<std::num::ParseIntError> for ForthErr {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::Parse(e)
    }
}

pub type Procedure = Box<dyn Fn(&mut ForthState) -> Result<(), ForthErr>>;

pub enum Word {
    Builtin(Procedure),
    Literal(i32),
}

pub struct ForthState<'a> {
    stack: Stack,
    mode: ForthMode,
    dictionary: HashMap<&'a str, Rc<Word>>,
}

impl<'a> ForthState<'a> {
    pub fn new(data_stack_capacity: usize) -> Self {
        let mut forth = Self {
            stack: Stack::new(data_stack_capacity),
            mode: ForthMode::Interpreting,
            dictionary: HashMap::new(),
        };

        forth.set_primitives();

        forth
    }

    pub fn stack(&self) -> &[stack::Data] {
        &self.stack.data()
    }

    pub fn eval(&mut self, line: String) -> Result<ForthReturn, ForthErr> {
        // a) Skip leading spaces and parse a name (see 3.4.1);
        for word_str in line.split_whitespace() {
            // TODO: do an interrupt to check for messages from CPU

            match word_str {
                "BYE" => {
                    return Ok(ForthReturn::Shutdown);
                }
                _ => {
                    // b) Search the dictionary name space (see 3.4.2).
                    let word = match self.find_word(word_str) {
                        Some(word) => word,
                        None => {
                            let i = self.convert_to_number(word_str)?;

                            Rc::new(Word::Literal(i))
                        }
                    };

                    match self.mode {
                        ForthMode::Interpreting => {
                            self.run_word(word)?;
                        }
                        ForthMode::Compiling => {
                            todo!("Compiling");
                        }
                    }
                }
            }
        }

        Ok(ForthReturn::Ok)
    }

    pub fn run_word(&mut self, word: Rc<Word>) -> Result<(), ForthErr> {
        match *word {
            Word::Builtin(ref built_in) => {
                built_in(self)?;
            }
            Word::Literal(ref lit) => {
                self.stack.push(*lit)?;
            }
        }
        Ok(())
    }

    pub fn find_word(&self, word: &str) -> Option<Rc<Word>> {
        match self.dictionary.get(word) {
            Some(word) => Some(word.clone()),
            None => None,
        }
    }

    pub fn convert_to_number(&self, word: &str) -> Result<i32, ForthErr> {
        Ok(word.parse::<i32>()?)
    }

    fn set_primitives(&mut self) {
        set_primitive!(self : "DOES>" => |context| {
            todo!();
        });

        set_primitive!(self : "CREATE" => |context| {
            todo!();
        });

        set_primitive!(self : "-" => |context| {
            let n1 = context.stack.pop()?;
            let n2 = context.stack.pop()?;

            context.stack.push(n1 - n2)?;

            Ok(())
        });

        set_primitive!(self : "+" => |context| {
            let n1 = context.stack.pop()?;
            let n2 = context.stack.pop()?;
            context.stack.push(n1 + n2)?;
            Ok(())
        });

        set_primitive!(self : "DUP" => |context |{
            let n = context.stack.pop()?;
            context.stack.push(n)?;
            context.stack.push(n)?;
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_subtracts() {
        let mut f = ForthState::new(333);
        f.eval("1 2 -".into()).unwrap();
        assert_eq!(1, f.stack()[0]);

        f.eval("-9 -".into()).unwrap();
        assert_eq!(-10, f.stack()[0]);
    }

    #[test]
    fn test_plus_adds() {
        let mut f = ForthState::new(333);
        f.eval("1 2 +".into()).unwrap();
        assert_eq!(3, f.stack()[0]);

        f.eval("1 +".into()).unwrap();
        assert_eq!(4, f.stack()[0]);
    }

    #[test]
    fn test_DUP_duplicates_top_of_stack() {
        let mut f = ForthState::new(333);
        f.eval("1 DUP".into()).unwrap();
        assert_eq!(1, f.stack()[0]);
        assert_eq!(1, f.stack()[1]);
    }

    #[test]
    fn test_bye_returns_exist() {
        assert_eq!(true, false);
    }

    #[test]
    fn variable() {
        let mut f = ForthState::new(333);

        f.eval("variable balance 123 balance ! balance @".into())
            .unwrap();

        assert_eq!(f.stack()[0], 123);
    }
}
