
#[allow(dead_code)]
pub mod tools{
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::Write;
    use std::path::PathBuf;
    use std::io::BufRead;
    use std::io::Seek;
    use crate::stack::Stack;
    use std::collections::HashMap;

    pub struct TextFile{
        file_path: PathBuf,
        file: File
    }
    impl TextFile{

        pub fn new(file_path: String) -> Result<TextFile, String> {
            if !file_exists(&file_path) {
                create_file(&file_path);
            }

            let file = match fs::OpenOptions::new().append(true).read(true).open(&file_path) {
                Ok(file) => file,
                Err(error) => return Err(format!("Error opening file: {}", error)),
            };

            Ok(TextFile {
                file_path: PathBuf::from(&file_path),
                file,
            })
        }

        pub fn push(&mut self, text: &str){
            self.file.write_all(text.as_bytes())
            .unwrap_or_else(|e|{
                println!("L'ajout du texte a la fin du fichier a echoué: {}", e);
            });
        }

        pub fn reset(&mut self, new_text: &str){
            self.file.set_len(0)
            .unwrap_or_else(|e|{
                println!("Le reset du texte a echoué: {}", e);
            });
            self.push(new_text);
        }

        pub fn erase(&self){
            fs::remove_file(&self.file_path)
            .unwrap_or_else(|e| {
                println!("Le fichier n'a pas été supprimé: {}", e);
            });
        }


        pub fn get_text(&mut self) -> String {
            let _ = self.file.seek(std::io::SeekFrom::Start(0));
            let mut result = String::new();
            let lines = io::BufReader::new(&self.file).lines();
            for line in lines {
                match line {
                    Ok(the_line) => {
                        result.push_str(&the_line);
                        result.push_str("\n");
                    }Err(e) => {
                        println!("Erreur lors de la lecture de la ligne {}", e);
                        return result;
                    }
                }
            }
            result
        }

        pub fn replace(&mut self, text_to_replace: &str, new_text: &str){
            let new_txt = self.get_text().replace(text_to_replace, new_text);
            self.reset(&new_txt);
        }
    }


    pub fn file_exists(file_path: &str) -> bool {
        fs::metadata(file_path).is_ok()
    }

    fn create_file(file_path: &str){
        let _ = File::create(&file_path).map_err(|e|{
            println!("Erreur lors de la creation du fichier {}: {}", file_path, e);
        });
    }


    pub struct Tools{
        authorized_char_for_variable: &'static str,
        operators: &'static str,
        operator_priority: HashMap<String, u8>,
        separators: &'static str
    }

    impl Tools{

        pub fn new() -> Tools{
            Tools{
                authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_",
                operators: "+-*/%",
                operator_priority: build_operator_priority(),
                separators: "(){}[],."
            }
        }

        pub fn is_valid_name(&self, name: &str) -> bool{
            for letter in name.chars(){
                if !self.authorized_char_for_variable.contains(letter){
                    return false;
                }
            }
            true
        }

        pub fn is_operator(&self, x: &str) -> bool{
            self.operators.contains(&x) && x != ""
        }

        pub fn convert_in_postfix_exp(&self, exp: Vec::<String>) -> Vec::<String>{
            let mut result = Vec::<String>::new();
            let mut stack = Stack::<String>::new();

            for e_elt in exp.iter(){
                let elt = String::from(e_elt);
                if self.is_operator(&elt) || elt == "("{
                    while !stack.is_empty() && *stack.val() != String::from("(") && self.operator_priority[&elt] <= self.operator_priority[&elt]{
                        result.push(stack.pop());
                    }
                    stack.push(elt);
                }else if elt == ")"{
                    while stack.val() != "(" {
                        result.push(stack.pop());
                    }
                    stack.pop();
                }else{
                    result.push(elt);
                }
            }
            while stack.size() != 0 {
                result.push(stack.pop());
            }
            result
        }
    
        
        pub fn is_separator(&self, chara: char) -> bool{
            self.separators.contains(chara)
        }

    }




    pub fn split(string: &str, splitter: &str) -> Vec::<String>{
        string.split(splitter).map(String::from).collect()
    }

    pub fn count_occur(string: &str, x: char) -> i32{
        let mut count = 0;
        for chara in string.chars(){
            if chara == x {
                count += 1;
            }
        }
        return count;
    }

    fn build_operator_priority() -> HashMap<String, u8>{
        let mut res = HashMap::<String, u8>::new();
        res.insert(String::from("+"), 1);
        res.insert(String::from("-"), 1);
        res.insert(String::from("*"), 2);
        res.insert(String::from("/"), 2);
        res.insert(String::from("("), 3);
        res.insert(String::from(")"), 2);
        res
    }
    
    pub fn is_par(elt: char) -> bool{
        return elt == '(' || elt == ')'
    }

    pub fn from_char_to_number(chara: &String) -> Option<i8> {
        if chara.len() != 3 || !chara.ends_with('\'') || !chara.ends_with('\''){
            return None
        }
        return Some(chara.chars().nth(1).unwrap() as i8)
    }

    pub fn extract_end_char(s: &mut String, chara: char) -> u32 {
        let mut result: u32 = 0;

        while s.len() > 0 {
            if !s.ends_with(chara) {
                return result
            }
            s.pop();
            result += 1;
        }
        
        result    
    }

    pub fn extract_start_char(s: &mut String, chara: char) -> u32 {
        let mut result: u32 = 0;

        while s.len() > 0 {
            if !s.starts_with(chara) {
                return result
            }
            s.remove(0);
            result += 1;
        }
        
        result    
    }

}