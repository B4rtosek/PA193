use semproject::*;
use std::process;
use std::io::{self, BufRead, Write};

pub fn ui(input: &str, output: &str) {
    let mut _input = input;
    let mut _output = output.to_string();
    let format = "base64"; // TODO delete this

    loop {
        println!("1 to encode");
        println!("2 to decode");
        println!("3 to set format (current: {})", format); //TODO getFormat
        println!("4 to set output stream (current: {})", _output);
        println!("5 to quit");
        print!("Your Choice: ");
        let _ = io::stdout().flush();
        let choice = io::stdin().lock().lines().next().unwrap().unwrap();
        delim();
        if choice == "1" {
            let encoded = ui_encode();
            print_output(encoded.as_str(), _output.as_str());
        } else if choice == "2" {
            let decoded = ui_decode();
            print_output(decoded.as_str(), _output.as_str());
        } else if choice == "3" {
            ui_format();
        } else if choice == "4" {
            _output = ui_output(_output.as_str());
        } else if choice == "5" {
            quit();
        } else {
            ui_confused();
            delim();
        }
    }
}

fn quit() {
    process::exit(0);
}

fn delim() {
    println!("--------------------------");
}

fn print_output(data: &str, output: &str) {
    if output == "stdout" {
        println!("Result: {}", data);
    } else {
        println!("Result printed to {}", output);
        //Print to file
    }
    delim();
}

fn ui_confused() {
    println!("Uh, Oh .. You mean what?");
}

fn ui_format() {
    let mut format = "base64";
    println!("Input / Output format setting");
    println!("Allowed: base64/hex/binary");
    println!("Current: {}", format);
    println!("1 to base64");
    println!("2 to hex");
    println!("3 to binary");
    println!("4 to go back");
    print!("New Format: ");
    let _ = io::stdout().flush();
    let choice = io::stdin().lock().lines().next().unwrap().unwrap();
    delim();
    if choice == "1" {
        format = "base64"; // TODO call setFormat
    } else if choice == "2" {
        format = "hex"; // TODO call setFormat
    } else if choice == "3" {
        format = "binary"; // TODO call setFormat
    } else if choice == "4" {
        
    } else {
        ui_confused();
        delim();
        ui_format();
    }
}

fn ui_output(output: &str) -> String {  
    println!("Output format setting");
    println!("Allowed: stdout/file");
    println!("Current: {}", output);
    println!("1 to stdout");
    println!("2 to file");
    println!("3 to go back");
    print!("New Output: ");
    let _ = io::stdout().flush();
    let choice = io::stdin().lock().lines().next().unwrap().unwrap();
    delim();
    if choice == "1" {
        return "stdout".to_string();
    } else if choice == "2" {
        print!("File: ");
        let _ = io::stdout().flush();
        let file = io::stdin().lock().lines().next().unwrap().unwrap();
        return file.to_string();
    } else if choice == "3" {
        return output.to_string();
    } else {
        ui_confused();
        delim();
        return ui_output(output);
    }
}

fn ui_decode() -> String {
    println!("Decoding with Bech32m");
    print!("Input: ");
    let _ = io::stdout().flush();
    let data = io::stdin().lock().lines().next().unwrap().unwrap(); // TODO return decoded data as string
    return "ASDF".to_string();
    // return decode(data.as_str()); 
}

fn ui_encode() -> String {
    println!("Encoding with Bech32m");
    print!("Input: ");
    let _ = io::stdout().flush();
    let data = io::stdin().lock().lines().next().unwrap().unwrap();
    let hrp = "bc";
    return "ASDF".to_string();
    //return encode(hrp, data.as_str()).unwrap().to_string();
}