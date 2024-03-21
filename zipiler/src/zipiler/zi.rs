use std::thread::{JoinHandle, spawn};
use super::collections::Queue;
use super::prog_manager::include::files::FUNCTIONSF;
use super::tokenizer::{include::{Token, TokenType}, tokenizer::Tokenizer};
use std::process::exit;
use super::program::Program;
use std::fs::File;
use super::prog_manager::include::{F_PATHS, files::*};
use std::io::{prelude::*, Write};
pub struct ZiLang {
    token_queue: Queue<Token>,
    tokenizing_thread: Vec<JoinHandle<()>>,
    keep_compile: bool,
    tools: Program,
    asm_files: Vec<File>
}


impl<'a> ZiLang {

    pub fn new() -> ZiLang {
        ZiLang{
            token_queue: Queue::new(),
            tokenizing_thread: Vec::new(),
            keep_compile: true,
            tools: Program::new(),
            asm_files: open_asm_files()
        }
    }

    pub fn compile(&mut self, input: File) {
        let mut tokenizer = Tokenizer::new(self);
        self.tokenizing_thread.push(spawn(move || 
            tokenizer.tokenize(input).unwrap_or_else(|e| {
                eprintln!("{e}");
                exit(1);
            })
        ));
        while self.keep_compile {
            if !self.token_queue.is_empty() {
                let token = self.token_queue.dequeue().expect("Queue empty");
                match self.tools.tokenize(token) {
                    Ok((asm, file_path)) => self.push_script(&asm, file_path),
                    Err(e) => panic!("{e}")
                };
            }
        }
        self.tools.end_prog();
        self.push_script(&self.tools.get_preload().clone(), FUNCTIONSF);
    }

    pub fn new_token(&mut self, token: Token) {
        self.token_queue.inqueue(token)
    }

    pub fn end_of_tokenizing_thread(&mut self) {
        self.keep_compile = false;
    }  

    pub fn new_group(&mut self, type_token: TokenType) {
        self.tools.new_group(type_token);
    }

    pub fn end_group(&mut self) {
        match self.tools.end_group() {
            Ok((end_txt, file_path)) => self.push_script(&end_txt, file_path),
            Err(e) => panic!("{e}")
        };
    }
    
    fn push_script(&mut self, txt: &str, file_path: usize) {
        self.asm_files[file_path].write(txt.as_bytes()).unwrap();
    }

}




fn open_asm_files() -> Vec<File> {
    let mut res = Vec::<File>::new();
    for f in F_PATHS.iter() {
        res.push(File::options().append(true).read(true).open(f).unwrap_or_else(|e| {
            panic!("Failed to open the file {f}: {e}")
        }))
    }
    replace_txt(&mut res, SCRIPTF, BASE_SCRIPTF);
    replace_txt(&mut res, FUNCTIONSF, BASEFUNCTIONSF);
    res
}

fn replace_txt(arr: &mut Vec<File>, f1: usize, f2: usize) {
    arr[f1].set_len(0).unwrap();
    let mut s = String::new();
    arr[f2].read_to_string(&mut s).unwrap();
    arr[f1].write(s.as_bytes()).unwrap();
}