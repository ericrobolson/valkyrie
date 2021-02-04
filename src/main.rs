use std::io::Write;
use std::{collections::HashMap, io};

fn main() {
    let mut forth = ForthState::new(i16::MAX as usize);

    loop {
        print!("valkyrie> ");
        io::stdout().flush().unwrap();

        // Do the reading
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(size) => match forth.eval_input(input) {
                Ok(result) => {
                    println!("OK -> STACK {:?}", forth.data_stack());
                }
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

#[derive(Debug, Copy, Clone)]
pub enum ForthErr {
    StackOverflow,
    StackUnderflow,
}

pub enum Word {
    Builtin(Box<dyn Fn(&mut ForthState) -> Result<(), ForthErr>>),
    Literal(i32),
}

pub struct ForthState<'a> {
    data_stack: Vec<i32>,
    data_stack_capacity: usize,
    mode: ForthMode,
    dictionary: HashMap<&'a str, Word>,
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
        let action: Box<dyn Fn(&mut ForthState) -> Result<(), ForthErr>> = {
            Box::new(|c| {
                let n1 = c.data_stack.pop().ok_or(ForthErr::StackUnderflow).unwrap();
                let n2 = c.data_stack.pop().ok_or(ForthErr::StackUnderflow).unwrap();

                c.data_stack.push(n1 + n2);

                Ok(())
            })
        };

        self.dictionary.insert("+", Word::Builtin(action));
    }

    pub fn data_stack(&self) -> &Vec<i32> {
        &self.data_stack
    }

    pub fn eval_input(&mut self, line: String) -> Result<(), String> {
        // 3.4 The Forth text interpreter (https://forth-standard.org/standard/usage)
        // TODO: Upon start-up, a system shall be able to interpret, as described by 6.1.2050 QUIT, Forth source code received interactively from a user input device.
        // Such interactive systems usually furnish a "prompt" indicating that they have accepted a user request and acted on it. The implementation-defined Forth prompt should contain the word "OK" in some combination of upper or lower case.

        // Text interpretation (see 6.1.1360 EVALUATE and 6.1.2050 QUIT) shall repeat the following steps until either the parse area is empty or an ambiguous condition exists:

        // a) Skip leading spaces and parse a name (see 3.4.1);
        for word_str in line.split_whitespace() {
            // b) Search the dictionary name space (see 3.4.2).
            let word = self.find_word(word_str);

            match word {
                Some(word) => {
                    // If a definition name matching the string is found:
                    match self.mode {
                        ForthMode::Interpreting => {
                            // TODO: 1) if interpreting, perform the interpretation semantics of the definition (see 3.4.3.2), and continue at a).
                            match word {
                                Word::Builtin(builtin) => {
                                    builtin(self); // TODO: resolve builtins.. Think we're getting closer but need to figure it out.
                                }
                            }
                        }
                        ForthMode::Compiling => {
                            // TODO: 2) if compiling, perform the compilation semantics of the definition (see 3.4.3.3), and continue at a).
                            todo!("Compiling?");
                        }
                    }
                }
                None => {
                    // c) If a definition name matching the string is not found, attempt to convert the string to a number (see 3.4.1.3).
                    match self.convert_to_number(word_str) {
                        Ok(word) => {
                            // If successful:
                            match self.mode {
                                ForthMode::Interpreting => {
                                    // if interpreting, place the number on the data stack, and continue at a);
                                    self.data_stack.push(word);
                                }
                                ForthMode::Compiling => {
                                    // TODO: 2) if compiling, compile code that when executed will place the number on the stack (see 6.1.1780 LITERAL), and continue at a);
                                    todo!("Compiling?");
                                }
                            }
                        }
                        Err(e) => {
                            // d)  If unsuccessful, an ambiguous condition exists (see 3.4.4).
                            // Just return an err
                            return Err(e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn find_word(&self, word: &str) -> Option<&Word> {
        self.dictionary.get(word)
    }

    pub fn convert_to_number(&self, word: &str) -> Result<i32, String> {
        match word.parse::<i32>() {
            Ok(i) => Ok(i),
            Err(e) => Err(format_error(word, e)),
        }
    }
}

fn format_error<E>(input: &str, error: E) -> String
where
    E: std::fmt::Debug,
{
    format!("In -> '{:?}' -> ERR '{:?}'", input, error)
}
