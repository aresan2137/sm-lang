use crate::pre_ast::{PreAST};

#[derive(Debug)]
pub struct Vars {
    pub vars: Vec<String>
}

pub fn find_var(pre: &PreAST) -> Vars {
    let mut vars = Vars { 
        vars: Vec::new() 
    };

    for func in &pre.functions {
        let mut i: usize = 1;
        while func.body.len() > i {
            let tok = &func.body[i];
            vars.vars.push(tok.value.to_string());

            i += 2;
        }

        i = 0;
        while func.body.len() > i {
            let mut tok = &func.body[i];
            if tok.value == "var" {
                i += 2;
                tok = &func.body[i];
                vars.vars.push(tok.value.to_string());
            }
            i += 1;
        }
    }

    return vars;
}