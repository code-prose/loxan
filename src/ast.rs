use std::io;
use std::env;

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


        self.define_ast(output_dir, String::from("Expr"), expr_array);

        Ok(())
    }

    fn define_ast(&mut self, output_dir: String, base_name: String, expression_types: Vec<String>) {
        todo!("")
    }
}
