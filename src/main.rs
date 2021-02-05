use std::io::Write;
use std::{io, rc::Rc};

mod dictionary;
use dictionary::Dictionary;
use stack::Stack;
mod stack;

fn main() {
    let mut forth = ForthState::new(i16::MAX as usize, 666);

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

#[derive(Debug, Clone, PartialEq)]
pub enum ForthMode {
    Interpreting,
    Compiling,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ForthReturn {
    Ok,
    Shutdown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForthErr {
    StackErr(stack::StackErr),
    DivideByZero,
    Parse(std::num::ParseIntError),
    DictionaryErr(dictionary::DictionaryErr),
    AccessedUndefinedAtAddr(usize),
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

impl From<dictionary::DictionaryErr> for ForthErr {
    fn from(de: dictionary::DictionaryErr) -> Self {
        Self::DictionaryErr(de)
    }
}

pub type Procedure = Box<dyn Fn(&mut ForthState) -> Result<(), ForthErr>>;

macro_rules! builtin_word {
    ($context:ident : $word:expr => $execution:expr) => {
        let action: Procedure = { Box::new($execution) };

        $context
            .dictionary
            .insert($word, Rc::new(Word::Builtin(action)))?;
    };
}

pub enum Word {
    Nothing,
    Builtin(Procedure),
    /// A custom, user defined word. If multiple words are chained together to make up this word, they are stored in the body and pushed to the call stack. The size of 13 is arbitrary, and open to change.
    Custom {
        body: [Rc<Word>; 13],
    },
    Literal(i32),
}
pub struct ForthState<'a> {
    stack: Stack<i32>,
    mode: ForthMode,
    dictionary: Dictionary<&'a str, Rc<Word>>,
}

impl<'a> ForthState<'a> {
    pub fn new(stack_capacity: usize, dictionary_capacity: usize) -> Self {
        let mut forth = Self {
            stack: Stack::new(stack_capacity),
            mode: ForthMode::Interpreting,
            dictionary: Dictionary::new(dictionary_capacity),
        };

        forth.reset();

        forth
    }

    pub fn reset(&mut self) {
        self.dictionary.clear();
        self.stack.clear();
        self.mode = ForthMode::Interpreting;
        self.set_primitives().unwrap();
    }

    pub fn stack(&self) -> &[i32] {
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
            Word::Custom { ref body } => {
                // Execute all queued methods
                for call in body.iter() {
                    match **call {
                        Word::Nothing => {}
                        _ => {
                            self.run_word(call.clone())?;
                        }
                    }
                }

                todo!()
            }
            Word::Nothing => {
                // Do nothing
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

    fn set_primitives(&mut self) -> Result<(), ForthErr> {
        builtin_word!(self : "DOES>" => |context| {
            todo!();
        });

        builtin_word!(self : "CREATE" => |context| {
            todo!();
        });

        builtin_word!(self : "VARIABLE" => |context| {
            todo!("https://forth-standard.org/standard/core/VARIABLE")
        });

        builtin_word!(self : "!" => |context| {
            todo!("https://forth-standard.org/standard/core/Store")
        });

        builtin_word!(self : "@" => |context| {
            // TODO: test
            // https://forth-standard.org/standard/core/Fetch
            let a_addr = context.stack.pop()?;
            match  context.dictionary.get_from_addr(a_addr as usize) {
                Some(value) => {
                    match **value {
                        Word::Literal(i) => {
                            context.stack.push(i)?;
                        },
                        _ => {
                            let value_type: String = match **value{
                                Word::Nothing => "Nothing".into(),
                                Word::Builtin(_) => "Builtin".into(),
                                Word::Custom {..} => "Custom".into(),
                                Word::Literal(lit) => format!("Literal: {:?}", lit),
                            };

                            todo!("Wrong type passed to put on stack! {:?}", value_type);
                        }
                    }

                },
                None => {
                    return Err(ForthErr::AccessedUndefinedAtAddr(a_addr as usize));
                }
            }

            Ok(())
        });

        builtin_word!(self : "-" => |context| {
            let n1 = context.stack.pop()?;
            let n2 = context.stack.pop()?;

            context.stack.push(n1 - n2)?;

            Ok(())
        });

        builtin_word!(self : "+" => |context| {
            let n1 = context.stack.pop()?;
            let n2 = context.stack.pop()?;
            context.stack.push(n1 + n2)?;
            Ok(())
        });

        builtin_word!(self : "*" => |context| {
            let n1 = context.stack.pop()?;
            let n2 = context.stack.pop()?;
            context.stack.push(n1 * n2)?;
            Ok(())
        });

        builtin_word!(self : "/" => |context| {
            let n1 = context.stack.pop()?;
            let n2 = context.stack.pop()?;
            if n2 == 0 {
                return Err(ForthErr::DivideByZero);
            }

            context.stack.push(n1 / n2)?;
            Ok(())
        });

        builtin_word!(self : "DUP" => |context |{
            let n = context.stack.pop()?;
            context.stack.push(n)?;
            context.stack.push(n)?;
            Ok(())
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_divides() {
        let mut f = ForthState::new(333, 343);
        f.eval("4 7 /".into()).unwrap();
        assert_eq!(1, f.stack()[0]);

        f.reset();

        f.eval("3 -9 /".into()).unwrap();
        assert_eq!(-3, f.stack()[0]);

        f.reset();

        assert_eq!(ForthErr::DivideByZero, f.eval("0 -9 /".into()).unwrap_err());
    }

    #[test]
    fn test_mul_multiplies() {
        let mut f = ForthState::new(333, 343);
        f.eval("4 7 *".into()).unwrap();
        assert_eq!(28, f.stack()[0]);

        f.eval("-9 *".into()).unwrap();
        assert_eq!(-252, f.stack()[0]);
    }

    #[test]
    fn test_sub_subtracts() {
        let mut f = ForthState::new(333, 343);
        f.eval("1 2 -".into()).unwrap();
        assert_eq!(1, f.stack()[0]);

        f.eval("-9 -".into()).unwrap();
        assert_eq!(-10, f.stack()[0]);
    }

    #[test]
    fn test_plus_adds() {
        let mut f = ForthState::new(333, 343);
        f.eval("1 2 +".into()).unwrap();
        assert_eq!(3, f.stack()[0]);

        f.eval("1 +".into()).unwrap();
        assert_eq!(4, f.stack()[0]);
    }

    #[test]
    fn test_DUP_duplicates_top_of_stack() {
        let mut f = ForthState::new(333, 343);
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
        let mut f = ForthState::new(333, 343);

        f.eval("variable balance 123 balance ! balance @".into())
            .unwrap();

        assert_eq!(f.stack()[0], 123);
    }
}
