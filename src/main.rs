use std::fs;
use std::process::Command;
use std::os::windows::process::CommandExt;

mod lexer;
mod pre_ast;
mod var_find;
mod ast;
mod cpp_maker;


fn main() {
    let lexer_output = lexer::tokenize("assets/code/main.sm");
    let pre = pre_ast::pre_parse(&lexer_output);
    let vars = var_find::find_var(&pre);
    let ast = ast::ast::parse_ast(&pre, &vars);
    let code = cpp_maker::make_cpp(&ast);

    fs::write("assets/out.cpp", code).expect("failed to save file");
    
    priti_print();
    //compile_and_run();
}

#[allow(dead_code)]
fn priti_print() {

    let full_command = "clang-format -i assets/out.cpp";

    let status = Command::new("cmd")
        .raw_arg("/C")
        .raw_arg(full_command)
        .status()
        .expect("failed to compile");

    if !status.success() {
        println!("error: priti print error: {:?}", status.code());
    }
}

#[allow(dead_code)]
fn compile_and_run() {
    let vcvars_path = r"C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat";
    let source_file = "assets/out.cpp";
    let output_file = "assets/program.exe";

    let full_command = String::from("call \"") + vcvars_path + "\" && " +
        "cl /EHsc /std:c++17 \"" + source_file + "\" /Fe:\"" + output_file + "\" && " +
        "\"" + output_file + "\"";

    println!("runing MSVC");

    let status = Command::new("cmd")
        .raw_arg("/C")
        .raw_arg(full_command)
        .status()
        .expect("failed to compile");

    if status.success() {
        println!("sucess");
    } else {
        println!("error: exit code: {:?}", status.code());
    }
}