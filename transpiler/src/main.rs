pub mod transpiler;

use std::{
    env::args,
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
};

use crate::transpiler::Transpiler;

const EXTENSION: &str = "lenga";

fn main() {
    let Some(input_path_str) = args().nth(1) else {
        println!("No input file provided");
        println!("Usage: {} <input_file>", args().next().unwrap());
        return;
    };

    let transpiler = Transpiler::new();

    let input_path = Path::new(&input_path_str);
    let mut file = match File::open(input_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {input_path_str}: {e}");
            return;
        }
    };
    let file_extension = input_path.extension().and_then(|s| s.to_str()).unwrap();

    if file_extension == EXTENSION {
        let mut content = Vec::new();
        BufReader::new(&mut file).read_to_end(&mut content).unwrap();
        let output_path = input_path.with_extension("");
        let file_extension = output_path.extension().and_then(|s| s.to_str()).unwrap();
        let output = transpiler
            .nodes_to_text(content, file_extension)
            .unwrap_or_else(|e| {
                eprintln!("Error parsing nodes file {input_path_str}: {e}");
                std::process::exit(1);
            });
        let mut output_file = File::create(output_path).unwrap();
        output_file.write_all(output.as_bytes()).unwrap();
    } else {
        let mut content = String::new();
        if let Err(e) = file.read_to_string(&mut content) {
            eprintln!("Error reading file {input_path_str}: {e}");
            return;
        }
        let output = transpiler.text_to_nodes(&content, file_extension).unwrap();
        let output_path = input_path.with_extension(format!("{file_extension}.{EXTENSION}"));

        let mut output_file = File::create(output_path).unwrap();
        output_file.write_all(&output).unwrap();
    }
}
