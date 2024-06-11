use serde::{Deserialize, Serialize};
use clap::Parser;

/**
* The structure of each command in the json file
*
* directory:
*   - required
*   - the working directory of the compilation
*
* file:
*   - required
*   - the translation unit source processed*
*
* command:
*   - required if arguments not present
*   - the compile command as a single shell-escaped string
*
* arguments:
*   - required if command not present
*   - the compile command argv as a list of strings, preferred to command
*
* output:
*   - optional
*   - the name of the output
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonCommand {
    pub directory:  String,
    pub command:    Option<String>,
    pub arguments:  Option<Vec<String>>,
    pub file:       String,
    pub output:     Option<String>
}

/**
 * The command-line input to the program
 */
#[derive(Parser)]
pub struct Cli {
    // location of compile_commands.json
    #[arg(short, long)]
    pub input:      std::path::PathBuf,

    // destination of converted compile_commands.json
    #[arg(short, long)]
    pub output:     std::path::PathBuf
}
