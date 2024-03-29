use super::include::*;

static ASM_TO_REMOVE: [&str; 2] = [
    "push rax            ; Then save the result\npop rax",
    "push rax\npop rax"
    ];


pub struct ExpTools {
    op_stack: Stack<String>,
    pf_exp: Vec<ExpTokenType>,
    operator_priority: HashMap<String, u8>,
    op_id_map: HashMap<String, u8>,
    esp_decal: i64,
    stars: i32
}

impl Tool for ExpTools {
    
    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
        match token.token_type {
            TokenType::Number => self.new_number(token.content.parse::<i128>().unwrap()),
            TokenType::RaiseComplexChar(v) => self.new_number(v as i128),
            TokenType::Operator => self.new_operator(token.content),
            TokenType::Symbol => self.new_parenthesis(token.content),
            TokenType::MemorySpot(nb_deref, stars, size) => self.new_cident(nb_deref, stars, size)?,
            TokenType::FuncCall(addr) => res = self.new_funccall(pm, addr)?,
            _ => panic_bad_token("expression", token)
        }
        Ok(res)
    }

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(ExpTools{
            op_stack : Stack::new(),
            pf_exp : Vec::new(),
            operator_priority: build_prio_map(),
            op_id_map: build_op_id(),
            esp_decal: 0,
            stars: 0
        })
    }

    /// The expressions raise the number of stars of the expression. The result is push on the stack
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        while self.op_stack.size() != 0 {
            self.push_op_val();
        }
        let asm = self.build_asm(pm);
        Ok((TokenType::RaiseExpression(self.stars), asm))
    }
    
}


impl ExpTools {

    fn new_funccall(&mut self, pm: &ProgManager, addr: usize) -> Result<String, String> {
        let f = pm.get_func_by_addr(addr);
        let stars = f.return_type().stars();
        self.new_cident(-1, stars as i32, pm.get_type_size(stars as i32, f.return_type().name()))?;
        Ok(String::from("\npush rax"))
    }

    fn new_operator(&mut self, content: String) {
        while !self.op_stack.is_empty() && 
              self.op_stack.val().unwrap() != "(" && 
              self.get_priority(self.op_stack.val().unwrap()) >= self.get_priority(&content){
            self.push_op_val();
        }
        self.op_stack.push(content);
    }

    fn new_number(&mut self, number: i128) {
        self.pf_exp.push(ExpTokenType::Number(number));
    }

    pub fn new_parenthesis(&mut self, par: String) {
        match &par as &str {
            "(" => self.op_stack.push(par),
            ")" => {
                while self.op_stack.val().unwrap() != "(" {
                    self.push_op_val();
                }
                self.op_stack.pop();
            }
            _ => panic!("Unknow parenthesis: {par}")
        } 
    }


    fn new_cident(&mut self, deref_t: i32, stars: i32, size: u8) -> Result<(), String> {
        if self.stars == 0 {
            self.stars = stars;
        }else if stars != 0 && stars != self.stars {
            return Err(String::from("Bad type."))
        }
        self.pf_exp.push(ExpTokenType::Ident(if deref_t==-1 {-1}else{1}, size));
        self.esp_decal += 8;
        Ok(())
    }

    fn get_priority(&self, op: &String) -> u8 {
        *self.operator_priority.get(op).unwrap_or_else(
            || panic!("This operator doesn't have priority yet: {op}")
        )
    }

    fn push_op_val(&mut self) {
        let val: u8 = *self.op_id_map.get(&self.op_stack.pop().unwrap()).unwrap();
        self.pf_exp.push(ExpTokenType::Operator(val));    
    }

    fn build_asm(&self, pm: &ProgManager) -> String {
        let mut nb_ident = 1;
        let mut res = String::new();
        res.push_str("\nmov rbp, rsp");
        for t in self.pf_exp.iter() {
            match t {
                ExpTokenType::Operator(op_code) => {
                    res.push_str(&format!("
mov r12, {op_code}        ; We found an operator, lets do the operation
pop r11             ; First operande
pop r10             ; Second operande
call _operation     ; call the operation
push rax            ; Then save the result"             
                    ))
    
                },
                ExpTokenType::Number(v) => {
                    res.push_str(&format!("
push {v}            ; We found a number, lets push it"
                    ))
                },
                ExpTokenType::Ident(is_ref, size) => {
                    res.push_str(&format!("      
mov rax, [rbp+{}]   ; We found an ident, its on the satck, lets keep it.
{}
push rax", self.esp_decal-nb_ident*8 as i64, if *is_ref==-1 {String::new()}else{pm.deref_var(*size as usize, 1)}
                    ));
                    nb_ident += 1;

                }
            }
        }
        nb_ident -=1 ;
        if nb_ident != 0 {
            res.push_str(&format!("
pop rax
add rsp, {}
push rax", nb_ident*8))
        }
        for a in ASM_TO_REMOVE.iter() {
            res = res.replace(a, "");
        }
        res
    }

}

fn build_prio_map() -> HashMap<String, u8>{
    let mut res = HashMap::<String, u8>::new();
    for op in vec!["%", "*", "/"].iter() {
        res.insert(String::from(*op), 4);
    }
    for op in vec!("<", "<=", ">", ">=", "==", "!=", "||", "&&").iter() {
        res.insert(String::from(*op), 2);
    }
    res.insert(String::from("+"), 3);
    res.insert(String::from("-"), 3);
    res.insert(String::from(")"), 4);
    res.insert(String::from("("), 5);
    res
}

fn build_op_id() -> HashMap<String, u8> {
    let mut res = HashMap::new();
    for (i, op) in OPERATORS.iter().enumerate() {
        res.insert(op.to_string(), i as u8);
    }
    res
}

enum ExpTokenType {
    Operator(u8),
    Number(i128),
    Ident(i8, u8)
}

