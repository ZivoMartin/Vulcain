use std::thread::spawn;
use super::tokenizer::{include::Token, tokenizer::Tokenizer};
use super::program::Program;
use std::fs::File;
use std::sync::mpsc::channel;





pub fn compile(input: File) -> Result<(), String>{
    let (sender, receiver) = channel::<Token>();
    let mut tokenizer = Tokenizer::new(sender);
    let mut prog = Program::new();
    spawn(move || 
        tokenizer.tokenize(input)
    );
    let mut keep_compile = true;
    while keep_compile {
        match receiver.recv(){
            Ok(token) => prog.tokenize(token)?,
            Err(_) => keep_compile = false
        };
    }
    prog.end_prog();
    Ok(())
}




