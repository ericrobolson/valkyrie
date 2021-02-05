use crate::{dictionary, id::Id, stack};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Interpreting,
    Compiling,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Return {
    Ok,
    Shutdown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextErr {
    StackErr(stack::StackErr),
    DivideByZero,
    Parse(std::num::ParseIntError),
    DictionaryErr(dictionary::DictionaryErr),
    AccessedUndefinedAtAddr(usize),
}

impl From<stack::StackErr> for ContextErr {
    fn from(se: stack::StackErr) -> Self {
        Self::StackErr(se)
    }
}

impl From<std::num::ParseIntError> for ContextErr {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::Parse(e)
    }
}

impl From<dictionary::DictionaryErr> for ContextErr {
    fn from(de: dictionary::DictionaryErr) -> Self {
        Self::DictionaryErr(de)
    }
}

pub type Procedure = Box<dyn Fn(&mut Context) -> Result<(), ContextErr>>;

macro_rules! builtin_word {
    ($context:ident : $word:expr => $execution:expr) => {
        let action: Procedure = { Box::new($execution) };

        $context
            .dictionary
            .insert(Some($word.into()), Rc::new(Word::Builtin(action)))?;
    };
}

pub enum Word {
    Nothing,
    Addr(dictionary::Addr),
    Builtin(Procedure),
    /// A custom, user defined word. If multiple words are chained together to make up this word, they are stored in the body and pushed to the call stack. The size of 13 is arbitrary, and open to change.
    Custom {
        body: [Rc<Word>; 13],
    },
    Literal(i32),
}
pub struct Context {
    stack: stack::Stack<i32>,
    mode: Mode,
    dictionary: dictionary::Dictionary<Id, Rc<Word>>,
    fsm: Fsm,
}

enum Fsm {
    Execute,
    GetVariable,
}

impl Context {
    pub fn new(stack_capacity: usize, dictionary_capacity: usize) -> Self {
        let mut forth = Self {
            fsm: Fsm::Execute,
            stack: stack::Stack::new(stack_capacity),
            mode: Mode::Interpreting,
            dictionary: dictionary::Dictionary::new(dictionary_capacity),
        };

        forth.reset();

        forth
    }

    pub fn reset(&mut self) {
        self.fsm = Fsm::Execute;
        self.dictionary.clear();
        self.stack.clear();
        self.mode = Mode::Interpreting;
        self.set_primitives().unwrap();
    }

    pub fn stack(&self) -> &[i32] {
        &self.stack.data()
    }

    pub fn eval(&mut self, line: String) -> Result<Return, ContextErr> {
        // a) Skip leading spaces and parse a name (see 3.4.1);

        // TODO: convert &strs to use bytes somehow

        for word_str in line.split_whitespace() {
            // TODO: do an interrupt to check for messages from CPU

            match self.fsm {
                Fsm::Execute => {
                    match word_str {
                        "BYE" => {
                            return Ok(Return::Shutdown);
                        }
                        "VARIABLE" => {
                            // https://forth-standard.org/standard/core/VARIABLE
                            // Idea: if you ever need to extend this, consider a FSM to wait for another input
                            self.fsm = Fsm::GetVariable;
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
                                Mode::Interpreting => {
                                    self.run_word(word)?;
                                }
                                Mode::Compiling => {
                                    todo!("Compiling");
                                }
                            }
                        }
                    }
                }
                Fsm::GetVariable => {
                    // add a value to the dict without a key.
                    let addr = self.dictionary.insert(None, Rc::new(Word::Nothing))?;
                    self.dictionary
                        .insert(Some(word_str.into()), Rc::new(Word::Addr(addr)))?;

                    // Switch back to execution mode
                    self.fsm = Fsm::Execute;
                }
            }
        }

        Ok(Return::Ok)
    }

    pub fn run_word(&mut self, word: Rc<Word>) -> Result<(), ContextErr> {
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
            Word::Addr(addr) => match self.dictionary.get_from_addr(addr) {
                Some(word) => {
                    todo!()
                }
                None => {
                    todo!()
                }
            },
        }
        Ok(())
    }

    pub fn find_word(&self, word: &str) -> Option<Rc<Word>> {
        match self.dictionary.get(word.into()) {
            Some(word) => Some(word.clone()),
            None => None,
        }
    }

    pub fn convert_to_number(&self, word: &str) -> Result<i32, ContextErr> {
        Ok(word.parse::<i32>()?)
    }

    fn set_primitives(&mut self) -> Result<(), ContextErr> {
        builtin_word!(self : "DOES>" => |context| {
            todo!();
        });

        builtin_word!(self : "CREATE" => |context| {
            todo!();
        });

        builtin_word!(self : "!" => |context| {
            todo!("https://forth-standard.org/standard/core/Store")
        });

        builtin_word!(self : "MEM-DICT" => |context| {
            todo!("make something that displays the entirety of the dictionary.");
            for kv in context.dictionary.dictionary(){
                //println!("DICT: {:?}", kv);
            }
            Ok(())
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
                                Word::Nothing=>"Nothing".into(),
                                Word::Builtin(_)=>"Builtin".into(),
                                Word::Custom{..}=>"Custom".into(),
                                Word::Literal(lit)=>format!("Literal: {:?}",lit),
                                Word::Addr(addr) => format!("addr{:?}", addr)
                            };

                            todo!("Wrong type passed to put on stack! {:?}", value_type);
                        }
                    }

                },
                None => {
                    return Err(ContextErr::AccessedUndefinedAtAddr(a_addr as usize));
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
                return Err(ContextErr::DivideByZero);
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
        let mut f = Context::new(333, 343);
        f.eval("4 7 /".into()).unwrap();
        assert_eq!(1, f.stack()[0]);

        f.reset();

        f.eval("3 -9 /".into()).unwrap();
        assert_eq!(-3, f.stack()[0]);

        f.reset();

        assert_eq!(
            ContextErr::DivideByZero,
            f.eval("0 -9 /".into()).unwrap_err()
        );
    }

    #[test]
    fn test_mul_multiplies() {
        let mut f = Context::new(333, 343);
        f.eval("4 7 *".into()).unwrap();
        assert_eq!(28, f.stack()[0]);

        f.eval("-9 *".into()).unwrap();
        assert_eq!(-252, f.stack()[0]);
    }

    #[test]
    fn test_sub_subtracts() {
        let mut f = Context::new(333, 343);
        f.eval("1 2 -".into()).unwrap();
        assert_eq!(1, f.stack()[0]);

        f.eval("-9 -".into()).unwrap();
        assert_eq!(-10, f.stack()[0]);
    }

    #[test]
    fn test_plus_adds() {
        let mut f = Context::new(333, 343);
        f.eval("1 2 +".into()).unwrap();
        assert_eq!(3, f.stack()[0]);

        f.eval("1 +".into()).unwrap();
        assert_eq!(4, f.stack()[0]);
    }

    #[test]
    fn test_DUP_duplicates_top_of_stack() {
        let mut f = Context::new(333, 343);
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
        let mut f = Context::new(333, 343);

        f.eval("variable balance 123 balance ! balance @".into())
            .unwrap();

        assert_eq!(f.stack()[0], 123);
    }
}
