mod types;

use types::*;
use serde_json::{Value, json};
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // location of compile_commands.json
    #[arg(short, long)]
    input:      std::path::PathBuf,

    // destination of converted compile_commands.json
    #[arg(short, long)]
    output:     std::path::PathBuf
}

fn main() {
    let args = Cli::parse();

    let in_filepath     = &args.input;
    let out_filepath    = &args.output;

    let file    = File::open(in_filepath).expect("couldn't open file");
    let reader  = BufReader::new(file);

    let json_data: Vec<JsonCommand> = serde_json::from_reader(reader).expect("couldn't parse file");

    let slashes = Regex::new(r"\\+").unwrap();
    let base    = Regex::new(r"C:+").unwrap();

    let processed_json: Vec<Value> = json_data
        .into_iter()
        .map(|data| {

            // todo: replace these with functions
            // replace C drive with mnt
            let directory = base.replace_all(&data.directory, "/mnt/c");
            let file = base.replace_all(&data.file, "/mnt/c");
            let command = base.replace_all(data.command.unwrap().as_str(), "/mnt/c");
            let vec = data.arguments;
            let arguments = vec.unwrap()
                                .into_iter()
                                .map(|arg| {})
            .replace_all(data.arguments.unwrap(), "/mnt/c");

            // next change slashes
            json!({
                "directory": directory,
                "command": slashes.replace_all(&command, "/").to_string(),
                "file": slashes.replace_all(&file, "/").to_string(),
                "output": slashes.replace_all(&data.output, "/").to_string(),
            })
        })
        .collect();

    let output_file = File::create(out_filepath).expect("couldn't create file");
    let mut writer = BufWriter::new(output_file);

    serde_json::to_writer_pretty(&mut writer, &processed_json)
        .expect("couldn't write json to file");

    println!("done");
}
