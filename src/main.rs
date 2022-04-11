use bech32m::*;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::{self, prelude::*, BufRead, Write};
use std::path::Path;
use std::process;

#[derive(PartialEq)]
enum InputType {
    STDIN,
    DATA,
    FILE,
}

#[derive(PartialEq)]
enum OutputType {
    STDOUT,
    FILE,
}

#[derive(PartialEq)]
enum Format {
    BASE64,
    HEX,
    BINARY,
}

#[derive(PartialEq)]
enum Operation {
    ENCODE,
    DECODE,
}

struct CliArgs {
    input_file: bool,
    hrp: bool,
    input: bool,
    output_file: bool,
    input_format: bool,
    output_format: bool,
    help: bool,
}

impl Default for CliArgs {
    fn default() -> CliArgs {
        CliArgs {
            input_file: false,
            hrp: false,
            input: false,
            output_file: false,
            input_format: false,
            output_format: false,
            help: false,
        }
    }
}

struct Cli {
    input_format: Format,
    output_format: Format,
    input: InputType,
    input_data: String,
    output: OutputType,
    output_path: String,
    operation: Option<Operation>,
    hrp: String,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli {
            input_format: Format::HEX,
            output_format: Format::HEX,
            input: InputType::STDIN,
            input_data: "".to_string(),
            output: OutputType::STDOUT,
            output_path: "".to_string(),
            operation: None,
            hrp: "rustafarian".to_owned(),
        }
    }
}

fn get_binary_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}

fn help() {
    println!("Bech32m Coding nad Decoding Tool - Rustafarians");
    println!("");
    println!("Usage:");
    println!("{} decode [options][paths...]", get_binary_name().unwrap());
    println!("{} encode [options][paths...]", get_binary_name().unwrap());
    println!("");
    println!("Options:");
    println!("  Input (default: stdin)");
    println!("    -i --input-file <file>             Selects input file");
    println!("    -d --input <data>                  Inputs the provided data");
    println!("");
    println!("  Output (default: stdout)");
    println!("    -o --output-file <file>            Selects output file");
    println!("");
    println!("  Input Format (default: hex, allowed: base64, binary, hex for encoding, bech32m for decoding)");
    println!("    -f --input-format <format>         Selects input format");
    println!("");
    println!("  Output Format (default: hex, allowed: base64, binary, hex for decoding, bech32m for encoding)");
    println!("    -a --output-format <format>        Selects output format");
    println!("  Hrp value (encode only, default: default_hrp");
    println!("    -r --hrp                           Sets hrp to use on encoding");
    println!("  -h --help                            Print usage");
}

fn input_error() {
    println!("Invalid use of arguments ...");
    help();
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut loaded_args = CliArgs {
        ..Default::default()
    };
    let mut settings = Cli {
        ..Default::default()
    };
    if args.len() < 2 {
        input_error();
    }

    match args[1].as_str() {
        "decode" => {
            settings.operation = Some(Operation::DECODE);
        }
        "encode" => {
            settings.operation = Some(Operation::ENCODE);
        }
        _ => {
            input_error();
        }
    }
    let mut i = 2;

    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--input-file" => {
                if loaded_args.input_file || loaded_args.input {
                    input_error();
                } else {
                    loaded_args.input_file = true;
                }
                settings.input = InputType::FILE;
                i += 1;
                if args.len() > i {
                    let path = Path::new(args[i].as_str());
                    if !path.exists() {
                        input_error();
                    } else {
                        let mut file = File::open(path).expect("File not found");
                        file.read_to_string(&mut settings.input_data)
                            .expect("Error while reading file");
                    }
                } else {
                    input_error();
                }
            }
            "-d" | "--input" => {
                if loaded_args.input || loaded_args.input_file {
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
            }
            "-o" | "--output-file" => {
                if loaded_args.output_file {
                    input_error();
                } else {
                    loaded_args.output_file = true;
                }
                settings.output = OutputType::FILE;
                i += 1;
                if args.len() > i {
                    settings.output_path = args[i].to_string();
                } else {
                    input_error();
                }
            }
            "-f" | "--input-format" => {
                if loaded_args.input_format {
                    input_error();
                } else {
                    loaded_args.input_format = true;
                }
                i += 1;
                if args.len() > i {
                    match args[i].as_str() {
                        "base64" => {
                            settings.input_format = Format::BASE64;
                        }
                        "hex" => {
                            settings.input_format = Format::HEX;
                        }
                        "binary" => {
                            settings.input_format = Format::BINARY;
                        }
                        _ => {
                            input_error();
                        }
                    }
                } else {
                    input_error();
                }
            }
            "-a" | "--output-format" => {
                if loaded_args.output_format {
                    input_error();
                } else {
                    loaded_args.output_format = true;
                }
                i += 1;
                if args.len() > i {
                    match args[i].as_str() {
                        "base64" => {
                            settings.output_format = Format::BASE64;
                        }
                        "hex" => {
                            settings.output_format = Format::HEX;
                        }
                        "binary" => {
                            settings.output_format = Format::BINARY;
                        }
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
            }
            "-r" | "--hrp" => {
                if settings.operation == Some(Operation::ENCODE) && loaded_args.hrp == false {
                    loaded_args.hrp = true;
                    i += 1;
                    if args.len() > i {
                        settings.hrp = args[i].to_string();
                    } else {
                        input_error();
                    }
                } else {
                    input_error();
                }
            }
            _ => {
                input_error();
            }
        }
        i += 1;
    }
    if settings.input == InputType::STDIN {
        let _ = io::stdout().flush();
        settings.input_data = io::stdin().lock().lines().next().unwrap().unwrap();
    }

    let result: Result<String, Error>;

    if settings.operation.unwrap() == Operation::DECODE {
        let inp_validation_result = valideh(settings.input_data.as_str());
        if inp_validation_result.result {
            match settings.output_format {
                Format::HEX => result = decode_hex(settings.input_data.as_str()),
                Format::BASE64 => result = decode_base64(settings.input_data.as_str()),
                Format::BINARY => result = decode_bin(settings.input_data.as_str()),
            };
        } else {
            println!("{}", inp_validation_result.reason);
            process::exit(1);
        }
    } else {
        match settings.input_format {
            Format::HEX => result = encode_hex(&*settings.hrp, settings.input_data.as_str()),
            Format::BASE64 => result = encode_base64(&*settings.hrp, settings.input_data.as_str()),
            Format::BINARY => result = encode_bin(&*settings.hrp, settings.input_data.as_str()),
        };
    }

    if result.is_err() {
        println!("{}", result.err().unwrap());
        process::exit(1);
    } else {
        let result_string = result.unwrap();

        if settings.output == OutputType::FILE {
            let path = Path::new(settings.output_path.as_str());
            let mut f = File::create(path).expect("Unable to create file");
            f.write_all(result_string.as_str().as_bytes())
                .expect("Unable to write data");
        } else {
            println!("{}", result_string.as_str());
        }
    }
}
