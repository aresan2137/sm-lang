use crate::ast::ast::{AST, Part};
use crate::ast::ast_tools::scan_tokens_until_str;
use crate::lexer::Token;
use crate::ast::tree::*;
use crate::var_find::*;

pub fn make_cpp(ast: &AST, vars: &Vars) -> String {
    let mut code = String::new();
    code.push_str("#include <iostream>\n#include <cstdint>\n#include <cmath>\n\n");

    for glob in &ast.glob_inlines  {
        code.push_str(&(glob.clone() + "\n"));
    }

    for func in &ast.functions {
        let ret = sm_type_to_cpp_type(&func.ret_type);
        let mut args_v = Vec::new();
        let mut i = 0;
        while i < func.args.len() {
            let mut argtyp = scan_tokens_until_str(&func.args, &mut i, ",");
            let nam = argtyp[argtyp.len() - 1].clone();
            argtyp.pop();
            args_v.push(format!("{} {}", sm_type_to_cpp_type(&argtyp), nam.value));
            i += 1; 
        }
        code.push_str(&format!("{} {}({});\n", ret, recreate_name(&func.name), args_v.join(",")));
    }

    code.push_str("\n");

    for func in &ast.functions {
        let ret = sm_type_to_cpp_type(&func.ret_type);
        let mut args_v = Vec::new();
        let mut i = 0;
        while i < func.args.len() {
            let mut argtyp = scan_tokens_until_str(&func.args, &mut i, ",");
            let nam = argtyp[argtyp.len() - 1].clone();
            argtyp.pop();
            args_v.push(format!("{} {}", sm_type_to_cpp_type(&argtyp), nam.value));
            i += 1; 
        }

        code.push_str(&format!("{} {}({})\n", ret, recreate_name(&func.name), args_v.join(",")));

        process_part(&func.body, &mut code, vars);

        code.push_str("\n\n");
    }
    code
}

fn process_part(part: &Part, code: &mut String, vars: &Vars) {
    match part {
        Part::Parts { parts } => {
            code.push_str("{\n");
            for part in parts {
                process_part(part, code, vars);
            }
            code.push_str("}\n");
        }
        Part::Var { var_type, var_name, value,is_bodyless } => {
            let cpp_t = sm_type_to_cpp_type(var_type);
            if *is_bodyless {
                code.push_str(&format!("{} {};\n", cpp_t, var_name));
            } else {
                code.push_str(&format!("{} {} = {};\n", cpp_t, var_name, translate_tree(value)));
            }
        }
        Part::Ret { value } => {
            code.push_str(&format!("return {};\n", translate_tree(value)));
        }
        Part::RetVoid {} => {
            code.push_str("return;\n");
        }
        Part::VarSet { var_name, value } => {
            code.push_str(&format!("{} = {};\n", var_name, translate_tree(value)));
        }
        Part::VarInc { var_name } => {
            code.push_str(&format!("{}++;\n", var_name));
        }
        Part::VarDec { var_name } => {
            code.push_str(&format!("{}--;\n", var_name));
        }
        Part::FuncCall { func_name, args }  => {
            let arg_strs: Vec<String> = args.iter().map(|a| translate_tree(a)).collect();
            code.push_str(&format!("{}({});\n", func_name, arg_strs.join(", ")));
        }
        Part::MethodCall { object, method_name, args } => {
            let obj_s = translate_tree(object);
            let arg_strs: Vec<String> = args.iter().map(|a| translate_tree(a)).collect();
            
            let mut is_ptr = false;
            for v in &vars.vars {
                if v.name == obj_s {
                    if v.v_type.contains("sp<") || v.v_type.contains("up<") {
                        is_ptr = true;
                    }
                    break;
                }
            }

            let op = if is_ptr { "->" } else { "." };
            code.push_str(&format!("{}{}{}({});\n", obj_s, op, method_name, arg_strs.join(", ")));
        }
        Part::CppBlock(_code) => {
            code.push_str(&format!("{}\n", _code));
        }
        Part::If { value } => {
            code.push_str(&format!("if ({})\n", translate_tree(value)));
        }
        Part::Else {} => {
            code.push_str("    else\n");
        }
        Part::While { value } => {
            code.push_str(&format!("while ({})\n", translate_tree(value)));
        }
    }
}


fn sm_type_to_cpp_type(sm_type: &Vec<Token>) -> String {

    let mut out = String::from("");

    for sm_ty in sm_type  {
        if sm_ty.value == "i8" { out += "int8_t "; }
        else if sm_ty.value == "i16" { out += "int16_t "; }
        else if sm_ty.value == "i32" { out += "int32_t "; }
        else if sm_ty.value == "i64" { out += "int64_t "; }

        else if sm_ty.value == "u8" { out += "uint8_t "; }
        else if sm_ty.value == "u16" { out += "uint16_t "; }
        else if sm_ty.value == "u32" { out += "uint32_t "; }
        else if sm_ty.value == "u64" { out += "uint64_t "; }

        else if sm_ty.value == "f32" { out += "float "; }
        else if sm_ty.value == "f64" { out += "double "; }

        else if sm_ty.value == "str" { out += "std::string "; }
        else if sm_ty.value == "bool" { out += "bool "; }
        else if sm_ty.value == "void" { out += "void "; }
        else if sm_ty.value == "auto" { println!("auto type detected and it's unrecomended"); return String::from("auto"); }

        else if sm_ty.value == "arr" { out += "std::vector"; }
        else if sm_ty.value == "up" { out += "std::unique_ptr"; }
        else if sm_ty.value == "sp" { out += "std::shared_ptr"; }

        else { out += &sm_ty.value };
    }

    return  out;
}

fn recreate_name(sm_name: &Vec<Token>) -> String {
    return sm_name.iter().map(|t| t.value.as_str()).collect();
}

