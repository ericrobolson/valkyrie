use std::{collections::HashMap, io, rc::Rc};
use std::{io::Write, num::ParseIntError};

fn main() {
    let mut forth = ForthState::new(i16::MAX as usize);

    loop {
        print!("valkyrie> ");
        io::stdout().flush().unwrap();

        // Do the reading
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(size) => match forth.eval_input(input) {
                Ok(result) => match result {
                    ForthReturn::Ok => {
                        println!("OK -> STACK {:?}", forth.data_stack());
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

pub enum ForthMode {
    Interpreting,
    Compiling,
}

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
    StackOverflow,
    StackUnderflow,
    Parse(std::num::ParseIntError),
}

pub enum Word {
    Builtin(Box<dyn Fn(&mut ForthState) -> Result<(), ForthErr>>),
    Literal(i32),
}

pub struct ForthState<'a> {
    data_stack: Vec<i32>,
    data_stack_capacity: usize,
    mode: ForthMode,
    dictionary: HashMap<&'a str, Rc<Word>>,
}

macro_rules! set_primitive {
    ($dictionary:ident : $word:expr => $execution:expr) => {
        let action: Box<dyn Fn(&mut ForthState) -> Result<(), ForthErr>> = { Box::new($execution) };

        $dictionary
            .dictionary
            .insert($word, Rc::new(Word::Builtin(action)));
    };
}

impl<'a> ForthState<'a> {
    pub fn new(data_stack_capacity: usize) -> Self {
        let mut forth = Self {
            data_stack_capacity,
            data_stack: Vec::with_capacity(data_stack_capacity),
            mode: ForthMode::Interpreting,
            dictionary: HashMap::new(),
        };

        forth.set_primitives();

        forth
    }

    fn set_primitives(&mut self) {
        set_primitive!(self : "-" => |context| {
            let n1 = context.data_stack.pop().ok_or(ForthErr::StackUnderflow)?;
            let n2 = context.data_stack.pop().ok_or(ForthErr::StackUnderflow)?;

            context.data_stack.push(n1 - n2);

            Ok(())
        });

        set_primitive!(self : "+" => |context| {
            let n1 = context.data_stack.pop().ok_or(ForthErr::StackUnderflow)?;
            let n2 = context.data_stack.pop().ok_or(ForthErr::StackUnderflow)?;

            context.data_stack.push(n1 + n2);

            Ok(())
        });

        set_primitive!(self : "*" => |context| {
            let n1 = context.data_stack.pop().ok_or(ForthErr::StackUnderflow)?;
            let n2 = context.data_stack.pop().ok_or(ForthErr::StackUnderflow)?;

            context.data_stack.push(n1 * n2);

            Ok(())
        });
    }

    pub fn data_stack(&self) -> &Vec<i32> {
        &self.data_stack
    }

    pub fn eval_input(&mut self, line: String) -> Result<ForthReturn, ForthErr> {
        // 3.4 The Forth text interpreter (https://forth-standard.org/standard/usage)
        // TODO: Upon start-up, a system shall be able to interpret, as described by 6.1.2050 QUIT, Forth source code received interactively from a user input device.
        // Such interactive systems usually furnish a "prompt" indicating that they have accepted a user request and acted on it. The implementation-defined Forth prompt should contain the word "OK" in some combination of upper or lower case.

        // Text interpretation (see 6.1.1360 EVALUATE and 6.1.2050 QUIT) shall repeat the following steps until either the parse area is empty or an ambiguous condition exists:

        // a) Skip leading spaces and parse a name (see 3.4.1);
        for word_str in line.split_whitespace() {
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
                self.data_stack.push(*lit);
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
        match word.parse::<i32>() {
            Ok(i) => Ok(i),
            Err(e) => Err(ForthErr::Parse(e)),
        }
    }
}

fn format_error<E>(input: &str, error: E) -> String
where
    E: std::fmt::Debug,
{
    format!("In -> '{:?}' -> ERR '{:?}'", input, error)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_Plus() {
        assert_eq!(true, false);
    }
}
