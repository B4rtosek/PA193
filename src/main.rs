use std::fs;
use std::env;
use std::process;
use std::path::Path;

enum InputType {
    STDIN,
    DATA,
    FILE,
}

enum OutputType {
    STDOUT,
    FILE,
}

enum Format {
    BASE64,
    HEX,
    BINARY,
}

struct CliArgs {
    input_file: bool,
    input: bool,
    output_file: bool,
    format: bool,
    help: bool,
}

impl Default for CliArgs {
    fn default() -> CliArgs {
        CliArgs {
            input_file: false,
            input: false,
            output_file: false,
            format: false,
            help: false,
        }
    }
}

struct Cli {
    format: Format,
    input: InputType,
    input_data: String,
    output: OutputType,
    output_path: Option<String>,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli {
            format: Format::BASE64,
            input: InputType::STDIN,
            input_data: "".to_string(),
            output: OutputType::STDOUT,
            output_path: None,
        }
    }
}

fn help() {
    println!("Bech32m Coding nad Decoding Tool - Rustafarians");
    println!("");
    println!("Usage: cargo run -- [options][paths...]");
    println!("");
    println!("Options");
    println!("  Input (default: stdin)");
    println!("    -i --input-file <file>             Selects input file");
    println!("    -d --input <data>                  Inputs the provided data");
    println!("  Output (default: stdout)");
    println!("    -o --output-file <file>            Selects output file");
    println!("  Format (default: base64, allowed: base64, binary, hex)");
    println!("    -f --format <format>               Selects input/output format");
    println!("  -h --help                            Print usage");

}

fn input_error() {
    help();
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut loaded_args = CliArgs { ..Default::default() };
    let mut settings = Cli { ..Default::default() };

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str(){
            "-i" | "--input-file" => {
                if loaded_args.input_file {
                    input_error();
                } else {
                    loaded_args.input_file = true;
                }
                settings.input = InputType::FILE;
                i += 1;
                if args.len() > i {
                    let path = Path::new(args[i].as_str());
                    if ! path.exists() {
                        input_error();
                    } else {
                        settings.input_data = fs::read_to_string(path).unwrap();
                    }
                } else {
                    input_error();
                }
            },
            "-d" | "--input" => {
                if loaded_args.input {
                    input_error();
                } else {
                    loaded_args.input = true;
                }
                settings.input = InputType::DATA;
                i += 1;
                if args.len() > i {
                    settings.input_data = args[i].to_string();
                } else {
                    input_error();
                }
            },
            "-o" | "--output-file" => {
                if loaded_args.output_file {
                    input_error();
                } else {
                    loaded_args.output_file = true;
                }
                settings.output = OutputType::FILE;
                i += 1;
                if args.len() > i {
                    settings.output_path = Some(args[i].to_string());
                    if ! Path::new(settings.output_path.unwrap().as_str()).exists() {
                        input_error();
                    }
                } else {
                    input_error();
                }
            },
            "-f" | "--format" => {
                if loaded_args.format {
                    input_error();
                } else {
                    loaded_args.format = true;
                }
                i += 1;
                if args.len() > i {
                    match args[i].as_str(){
                        "base64" => {
                            settings.format = Format::BASE64;
                        },
                        "hex" => {
                            settings.format = Format::HEX;
                        }, 
                        "binary" => {
                            settings.format = Format::BINARY;
                        }, 
                        _ => {
                            input_error();
                        }
                    }
                } else {
                    input_error();
                }
            }
            "-h" | "--help" => {
                if loaded_args.help {
                    input_error();
                } else {
                    loaded_args.help = true;
                }
                help();
                process::exit(0);
            },
            _ => {
                input_error();
            }
        }
        i += 1;
    }
    
    println!("{}", settings.input_data);
    
}
