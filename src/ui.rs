use semproject::*;
use std::process;
use std::io::{self, BufRead, Write};

pub fn ui() {
  loop {
    question();
  }
}

fn quit() {
    process::exit(0);
}

fn delim() {
    println!("--------------------------");
}

fn question() {
    println!("Press \'E\' to encode");
    println!("Press \'D\' to decode");
    println!("Press \'Q\' to quit");
    print!("Your Choice: ");
    let _ = io::stdout().flush();
    let choice = io::stdin().lock().lines().next().unwrap().unwrap();
    delim();
    if choice == "E" || choice == "e" {
        ui_encode();
    } else if choice == "D" || choice == "d" {
        ui_decode();
    } else if choice == "Q" || choice == "q" {
        quit();
    } else {
        println!("Uh, Oh .. You mean what?");
        question();
    }
}

fn ui_decode() {
    println!("Decoding with Bech32m");
    print!("Input: ");
    let _ = io::stdout().flush();
    let data = io::stdin().lock().lines().next().unwrap().unwrap();
    let decoded = decode(data.as_str());
    println!("Result: {:?}", decoded);
    delim();
}

fn ui_encode() {
    println!("Encoding with Bech32m");
    print!("Input: ");
    let _ = io::stdout().flush();
    let data = io::stdin().lock().lines().next().unwrap().unwrap();
    let hrp = "bc";
    let encoded = encode(hrp, data.as_str());
    println!("Result: {}", encoded.unwrap());
    delim();
}