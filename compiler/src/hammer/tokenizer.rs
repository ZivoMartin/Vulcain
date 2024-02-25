use std::collections::HashMap;


#[derive(Eq, Hash, PartialEq, Debug)]
enum TokenType {
    // Primitive Token
    Ident,
    Number,
    Type,
    Symbol,
    // Keyword,

    // Token group
    Program,
    Instruction,
    Value,              
    ComplexIdent,       
    Expression,         
    FuncCall,           // Ident-Tuple
    Tuple,              // (Expression, Expression, ... , Expression)
    SerieExpression,    
    Affectation        // = Expression
}

impl Copy for TokenType {}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
}


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    string: String
}

struct Node {
    type_token: TokenType,
    groups: Vec<Node>, 
    sons: Vec<Node>,
    can_end: bool
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        other.type_token == self.type_token
    }
}

impl Node {

    /// Build a new node wich has to be builded.
    fn new(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node{type_token, groups, sons, can_end: true}
    }

    /// Build a leaf, a leaf has to be builded
    fn leaf(type_token: TokenType) -> Node {
        Node{type_token, sons: vec!(), groups: vec!(), can_end: false}
    }

    /// Build a new node wich can end the building of the group.
    fn new_end(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node{type_token, groups, sons, can_end: true}
    }

    fn end_inst() -> Node {
        Node::leaf(TokenType::Symbol)
    }
}

pub struct Tokenizer {
    group_map: HashMap<TokenType, Node>,
}

impl<'a> Tokenizer {

    pub fn new() -> Tokenizer {
        let mut res = Tokenizer{
            group_map: HashMap::<TokenType, Node>::new(),
        };
        res.init_token_groups();
        res
    }

    pub fn tokenize(&self, input: String) -> Result<Vec<Token>, String> {
        let son_vec = self.get_son_array(self.group_map.get(&TokenType::Program).unwrap());
        let mut chars = input.chars();
        while let Some(c) = chars.next() {
            
        }
        Ok(Vec::new())
    } 

    fn get_next_token(&self, c: char, son_vec: &Vec<&Node>, current_token: &mut String, mut chars: std::str::Chars) -> Result<String, String> {
        let mut current_token = String::new();
        if is_number(c) {
            if son_vec.contains(&self.group_map.get(&TokenType::Number).unwrap()) {
                current_token.push(c);
                while let Some(c) = chars.nth(0) {
                    if is_number(c) {
                        current_token.push(c);
                        chars.next();
                    }else{
                        continue
                    }
                }
                Ok(current_token)
            }else{
                return Err(format!("You can't put a number here."))
            }
        }else if is_letter(c) {
            let mut can_be_ident = false;
            let mut error = true;
            for son in son_vec.iter() {
                if son.type_token == TokenType::Ident || son.type_token ==  TokenType::Type {
                    can_be_ident = son.type_token == TokenType::Ident;
                    error = false;
                    
                }
            }
            if error {
                return Err(format!("You can't put a letter here"));
            }
            while let Some(c) = chars.nth(0) {
                if is_number(c) {
                    if !can_be_ident {
                        break
                    } 
                    
                }
                current_token.push(c);
                chars.next();
            }
            Ok(current_token)

        }else{
            while let Some(c) = chars.nth(0) {
                if is_sign(c) {
                    current_token.push(c);
                    chars.next();
                }else{
                    break;
                }
            }
            Ok(current_token)
        }
    }

    fn get_son_array(&self, node: &'a Node) -> Vec<&'a Node> {
        let mut res = Vec::new();
        for son in node.sons.iter() {
            res.push(son);
        }
        for group in node.groups.iter() {
            res.append(&mut self.get_son_array(group))
        }
        res
    }




    fn init_token_groups(&mut self) {
        self.group_map.insert(
            TokenType::Tuple,
            Node::new(
                TokenType::Tuple,
                vec!(),
                vec!(
                    Node::new(
                        TokenType::Symbol, // ( 
                        vec!(Node::new(
                            TokenType::SerieExpression,
                                vec!(),
                                vec!(
                                    Node::leaf(TokenType::Symbol) // )
                                )
                            )
                        ), 
                        vec!()
                    )
                )
            )
        );

        
        self.group_map.insert(
            TokenType::SerieExpression,
            Node::new_end(
                TokenType::SerieExpression,
                vec!(
                    Node::new(
                        TokenType::Expression,
                        vec!(),
                        vec!(
                            Node::leaf(
                                TokenType::Symbol, // ,
                            )
                        )
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::ComplexIdent,
            Node::new(
                TokenType::ComplexIdent,
                vec!(),
                vec!(
                    Node::new_end(
                        TokenType::Ident,
                        vec!(),
                        vec!(
                            Node::new(
                                TokenType::Symbol, // [
                                vec!(
                                    Node::new(
                                        TokenType::Expression,
                                        vec!(),
                                        vec!(
                                            Node::leaf(
                                                TokenType::Symbol, // ]
                                            )
                                        )
                                    )
                                ),
                                vec!()
                            )
                        )
                    ),
                    Node::new(
                        TokenType::Symbol, // $
                        vec!(
                            Node::leaf(
                                TokenType::ComplexIdent
                            )
                        ),
                        vec!()
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::FuncCall,
            Node::new(
                TokenType::FuncCall,
                vec!(),
                vec!(
                    Node::new(
                       TokenType::Ident,
                       vec!(
                           Node::leaf(
                             TokenType::Tuple
                           )
                       ),
                       vec!()
                    )
                )
            )
        );
        
        self.group_map.insert(
            TokenType::Value,
            Node::new(
                TokenType::Value,
                vec!(
                    Node::leaf(
                        TokenType::FuncCall
                    ),
                    Node::leaf(
                        TokenType::ComplexIdent
                    ),
                    Node::leaf(
                        TokenType::Number
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::Expression,
            Node::new(
                TokenType::Expression,
                vec!(
                    Node::new_end(
                        TokenType::Value,
                        vec!(
                            Node::new(
                                TokenType::Symbol,  // Operateur
                                vec!(
                                    Node::leaf(
                                        TokenType::Expression
                                    )
                                ),
                                vec!()
                            )
                        ),
                        vec!()
                    )
                ),
                vec!(
                    Node::new(
                        TokenType::Symbol,  //(
                        vec!(
                            Node::leaf(
                                TokenType::Expression
                            )
                        ),
                        vec!()
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::Affectation,
            Node::new(
                TokenType::Affectation,
                vec!(
                    Node::new(
                        TokenType::Symbol, // =
                        vec!(
                            Node::leaf(
                                TokenType::Expression 
                            )
                        ),
                        vec!()
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::Instruction,
            Node::new(
                TokenType::Instruction,
                vec!(),
                vec!(
                    Node::leaf(
                        TokenType::Symbol, // }
                    ),
                    Node::new(
                        TokenType::Type, 
                        vec!(
                            Node::new(
                                TokenType::Ident,
                                vec!(
                                    Node::new(
                                        TokenType::Affectation,
                                        vec!(),
                                        vec!(Node::end_inst())
                                    )
                                ),
                                vec!(Node::end_inst())
                            )
                        ),
                        vec!()
                    ),
                    Node::new(
                        TokenType::Ident,
                        vec!(
                            Node::new(
                                TokenType::Affectation,
                                vec!(),
                                vec!(Node::end_inst())
                            ),
                            Node::new(
                                TokenType::Tuple,
                                vec!(),
                                vec!(Node::end_inst())
                            ),
                        ),
                        vec!()
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::Program,
            Node::new_end(
                TokenType::Program, 
                vec!(
                    Node::new(
                        TokenType::Instruction,
                        vec!(
                            Node::leaf(
                                TokenType::Program,
                            )
                        ),
                        vec!()
                    )
                ), 
                vec!(
                    Node::new(
                        TokenType::Symbol,  // \n
                        vec!(
                            Node::leaf(
                                TokenType::Program,
                            )
                        ),
                        vec!()
                    )
                )
            )
        );
    }
}



fn is_sign(c: char) -> bool {
    !is_number(c) && !is_letter(c)
}

fn is_number(c: char) -> bool {
    (c as u8) < 58 && (c as u8) >= 48
}

fn is_letter(c: char) -> bool {
    (c as u8) >= 65 && (c as u8) <= 122 && !((c as u8) >= 91 && (c as u8) <= 96)  
}