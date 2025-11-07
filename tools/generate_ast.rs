use std::env;
use std::io::{self, Write};
use std::fs;

fn define_ast(output_directory: String, base_name: String, types: Vec<String>) -> io::Result<()> {
    let mut path: String = format!("{}/{}.rs", output_directory, base_name.to_lowercase());

    let mut ast_string: String = String::from(
        "use crate::token::Token;\n\
        use crate::value::Value;\n\
        \n\
        "
    );

    ast_string += format!("pub enum {} {{", base_name).as_str();
    
    let mut struct_strings: Vec<String> = vec![];
    let mut struct_names: Vec<String> = vec![];
    for struct_def in &types {
        let mut parts: Vec<String> = struct_def
            .split(":")
            .map(|s| s.to_string())
            .collect();

        let struct_name: String = parts[0].trim().to_string().clone();
        struct_names.push(struct_name.clone());

        let struct_params: Vec<String> = parts[1]
            .split(",")
            .map(|s| s.to_string())
            .collect();
        
        let mut struct_string: String = format!("\t{} {{", struct_name);
        for struct_param in struct_params {
            let param_split: Vec<String> = struct_param
                .split(";")
                .map(|s| s.to_string())
                .collect();
            
            struct_string += format!("\n\t\t{}: {},", param_split[1], param_split[0]).as_str();
        }
        struct_string += "\n\t},";

        struct_strings.push(struct_string);
    }

    for struct_string in struct_strings {
        ast_string += format!("\n{}\n", struct_string).as_str();
    }
    ast_string += "}\n\n";

    let mut visitor_string = String::from(
        "pub trait Visitor {\n\
        \ttype Result;\n"
    );
    for struct_name in &struct_names {
        visitor_string += format!("\n\tfn visit_{}(&mut self, {}: &{}) -> Self::Result;", struct_name.to_lowercase(), struct_name.to_lowercase(), base_name).as_str();
    }
    visitor_string += "\n}\n\n";
    ast_string += visitor_string.as_str();

    ast_string += format!(
        "impl {} {{\n\
        \tpub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {{\n\
        \t\tmatch self {{\n",
    base_name).as_str();

    for struct_def in &types {
        let mut parts: Vec<String> = struct_def
            .split(":")
            .map(|s| s.to_string())
            .collect();

        let struct_name: String = parts[0].trim().to_string().clone();
        struct_names.push(struct_name.clone());

        let struct_params: Vec<String> = parts[1]
            .split(",")
            .map(|s| s.to_string())
            .collect();
        
        let mut visitor_impl_string = format!(
            "\t\t\tExpr::{} {{",
        struct_name);

        for struct_param in struct_params {
            let param_split: Vec<String> = struct_param
                .split(";")
                .map(|s| s.to_string())
                .collect();
            
            visitor_impl_string += format!("{}, ", param_split[1]).as_str();
        }
            
        visitor_impl_string += format!(
            " }} => {{\n\
            \t\t\t\tvisitor.visit_{}(self)\n\
            \t\t\t}}\n",
        struct_name.to_lowercase()).as_str();

        ast_string += visitor_impl_string.as_str();
    }

    ast_string += "\t\t}\n\t}\n}";

    let mut file = fs::OpenOptions::new()
        .write(true) // Enable append mode
        .create(true) // Create the file if it doesn't exist
        .open(path)?;

    file.write_all(&ast_string.into_bytes());
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len != 2 {
        println!("Usage: generate_ast [output_directory]");
    }

    let output_dir: String = args[1].clone();
    define_ast(output_dir, String::from("Expr"), vec![
        String::from("Binary     :Box<Expr>;left,Token;operator,Box<Expr>;right"),
        String::from("Grouping   :Box<Expr>;expression"),
        String::from("Literal    :Value;value"),
        String::from("Unary      :Token;operator,Box<Expr>;right"),
    ]);
}