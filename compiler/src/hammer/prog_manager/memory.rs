use super::{include::*, prog_manager::ProgManager};



impl ProgManager {
    pub fn is_function(&self, name: &str) -> bool {
        self.func_name_map.contains_key(name) && !self.func_name_map.get(name).unwrap().is_empty()
    }

    pub fn new_function(&mut self, name: String, args: Vec<Type>, return_type: Type) {
        let f = Function::new(self.si(), name.clone(), args, return_type);
        match self.func_name_map.get_mut(&name) {
            Some(s) => s.push(self.si()),
            _ => {self.func_name_map.insert(name, Stack::init(self.si()));}
        };
        self.func_map.insert(self.si(), f);
        self.stack_index += 8;
    }

    pub fn get_func_addr(&self, name: &str) -> usize {
        self.get_func_by_name(name).unwrap().addr()
    }

    pub fn get_func_by_addr(&self, addr: usize) -> &Function {
        self.func_map.get(&addr).unwrap()
    }

    pub fn get_func_by_name(&self, name: &str) -> Result<&Function, String> {
        match self.func_name_map.get(name) {
            Some(s) => Ok(
                self.get_func_by_addr(*(
                    s.val().unwrap()))
                ), 
            _ => Err(format!("The function {name} doesn't exists."))
        }
    }

    pub fn new_var(&mut self, name_type: String, name: String, stars: u32) -> usize {
        let size = self.get_type_size_with_type_name(&name_type); 
        self.var_map.insert(
            self.si(),
            VariableDefinition{
                name: name.clone(),
                type_var: Type::new(name_type, size, stars),
                addr: self.si()
            }
        );
        if self.var_name_map.contains_key(&name) {
            self.var_name_map.get_mut(&name).unwrap().push(self.si());
        }else{
            self.var_name_map.insert(
                name,
                Stack::init(self.si())
            );
        }
        let res = self.si();
        self.jump_stack.val_mut().expect("jump stack empty").add_addr(self.si());
        self.stack_index += size as usize;
        res
    } 

    pub fn get_var_def_by_name(&self, name: &String) -> Result<&VariableDefinition, ()> {
        let addr = match self.var_name_map.get(name) {
            Some(stack) => stack.val().expect("The stack of a var name is empty"),
            _ => return Err(()) 
        };
        Ok(self.var_map.get(addr).unwrap())
    }
    
    pub fn get_var_def(&self, addr: &usize) -> Result<&VariableDefinition, ()> {
        match self.var_map.get(addr) {
            Some(res) => Ok(res),
            _ => Err(())
        }
    }
}