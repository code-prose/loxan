use std::env;
use std::fs::File;
use std::io;
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

        // start enum
        let base_enum_name = format!("enum {base_name} {{\n");
        let base_enum_name_bytes = base_enum_name.clone().into_bytes();
        file.write_all(&base_enum_name_bytes)?;

        // expr defs
        for expr_type in expression_types {
            let mut enum_name_iter = expr_type.split_terminator(':');
            let enum_name = match enum_name_iter.next() {
                Some(v) => v,
                None => {
                    println!("Failed to generate AST from expr vec");
                    std::process::exit(65)
                }
            }
            .to_string();

            let enum_fields = match enum_name_iter.next() {
                Some(v) => v,
                None => {
                    println!("Failed to generate AST from expr vec");
                    std::process::exit(65)
                }
            }
            .to_string();

            self.define_type(&mut file, &base_enum_name, enum_name, enum_fields);
        }

        file.write_all(b"}\n")?;
        Ok(())
    }

    fn define_type(
        &mut self,
        file: &mut std::fs::File,
        base_enum_name: &String,
        enum_name: String,
        enum_fields: String,
    ) {
        todo!("")
    }
}
