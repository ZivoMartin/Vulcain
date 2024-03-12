pub use crate::hammer::{
                        {memory::Memory, include::VariableDefinition},
                        program::{Tool, panic_bad_token},
                        tokenizer::include::{TokenType, Token, OPERATORS}
                    };
pub use crate::tools::collections::Stack;
pub use std::collections::HashMap;
pub static NO_TYPE: i32 = -2;
