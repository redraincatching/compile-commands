use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Debug, Deserialize, Serialize)]
struct JsonCommand {
    directory:  String,
    command:    String,
    file:       String,
    output:     String
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // TODO: swap this out for the clap crate so i can actually use args, and throw in checking
    let in_filepath   :&str = &args[1];
    let out_filepath  :&str = &args[2];

    let file    = File::open(in_filepath).expect("couldn't open file");
    let reader  = BufReader::new(file);

    let json_data: Vec<JsonCommand> = serde_json::from_reader(reader).expect("couldn't parse file");

    let slashes = Regex::new(r"\\+").unwrap();
    let base    = Regex::new(r"C:+").unwrap();

    let processed_json: Vec<Value> = json_data
        .into_iter()
        .map(|data| {
            let directory = base.replace_all(&data.directory, "/mnt/c");
            let file = base.replace_all(&data.file, "/mnt/c");
            let command = base.replace_all(&data.command, "/mnt/c");

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
