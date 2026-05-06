use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;

pub struct GenerateAst;

impl GenerateAst {
    fn main(&mut self, args: Vec<String>) -> io::Result<()> {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            println!("Usage: generate_ast {{ouput_directory}}");
            std::process::exit(64);
        }
        let output_dir = args[1].clone();
        let mut expr_array = Vec::new();
        expr_array.push("Binary   : Expr left, Token operator, Expr right".to_string());
        expr_array.push("Grouping : Expr expression".to_string());
        expr_array.push("Literal  : Object value".to_string());
        expr_array.push("Unary    : Token operator, Expr right".to_string());

        let _res = self.define_ast(output_dir, String::from("Expr"), expr_array);

        Ok(())
    }

    fn define_ast(
        &mut self,
        output_dir: String,
        base_name: String,
        expression_types: Vec<String>,
    ) -> io::Result<()> {
        let path = format!("{output_dir}/{base_name}.rs");
        let mut file = File::create(path)?;
        file.write_all(b"use crate::tokens::{Token, TokenType, Literal}\n")?;
        file.write_all(b"\n")?;

        let class_name = format!("enum {base_name} {{\n").into_bytes();
        file.write_all(&class_name)?;
        file.write_all(b"}\n")?;

        todo!("")
    }
}
