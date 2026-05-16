use crate::pre_ast::{PreAST};

#[derive(Debug)]
pub struct VarData {
    pub name: String,
    pub v_type: String,
}

#[derive(Debug)]
pub struct Vars {
    pub vars: Vec<VarData>
}

pub fn find_var(pre: &PreAST) -> Vars {
    let mut vars_vec = Vec::new();

    for func in &pre.functions {
        let mut i = 0;
        let body = &func.body;
        while i < body.len() {
            if body[i].value == "var" {
                i += 1;
                let mut type_tokens = Vec::new();
                
                while i < body.len() && body[i].value != "=" && body[i].value != ";" {
                    type_tokens.push(body[i].value.clone());
                    i += 1;
                }

                if type_tokens.len() >= 2 {
                    let v_name = type_tokens.pop().unwrap();
                    let v_type = type_tokens.join("");
                    vars_vec.push(VarData { name: v_name, v_type });
                }
            } else {
                i += 1;
            }
        }
    }

    return Vars { vars: vars_vec };
}