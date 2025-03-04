mod parser;
mod logical_plan;
mod physical_plan;
mod errors;
mod join;
mod scanner;
mod select;
mod filter;
mod order_by;

use parser::parse_pipe_sql;
use physical_plan::convert_to_physical_plan;
use std::io::{self, Write};

fn main() {
    println!("🚀 BigQuery Pipe SQL Parser 🚀");

    loop {
        print!("BigQuery |> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        match parse_pipe_sql(&input) {
            Ok(logical_plan) => {
                println!("✅ Logical Plan:\n{:#?}", logical_plan);
                let physical_plan = convert_to_physical_plan(&logical_plan);
                println!("✅ Physical Plan:\n{:#?}", physical_plan);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    }
}
