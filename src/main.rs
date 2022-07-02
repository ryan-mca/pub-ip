use std::fs::File;
use std::io::{stdout, Write};
use curl::easy::Easy;
use std::env;
use std::process::exit;

use clap::{Arg, App};

fn writev4() {
    let mut file = File::create("pub-ipv4.txt")
        .expect("Could not create file");
    let mut easy = Easy::new();

    easy.url("ipv4.icanhazip.com").unwrap();
    easy.write_function(move |data| {
        file.write_all(data).expect("Failed to write data");
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    exit(0);
}

fn writev6() {
    let mut file = File::create("pub-ipv6.txt")
        .expect("Could not create file");
    let mut easy = Easy::new();

    easy.url("ipv6.icanhazip.com").unwrap();
    easy.write_function(move |data| {
        file.write_all(data).expect("Failed to write data");
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    exit(0);
}

fn printv4() {
    let mut easy = Easy::new();
    easy.url("ipv4.icanhazip.com").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    exit(0);
}

fn printv6 () {
    let mut easy = Easy::new();
    easy.url("ipv6.icanhazip.com").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    exit(0); 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    if command == "--printv4" {
        printv4()
    }

    if command == "--printv6" {
        printv6()
    }

    if command == "-p4" {
        printv4()
    }

    if command == "-p6" {
        printv6()
    }

    if command == "--writev4" {
        writev4()
    }

    if command == "--writev6" {
        writev6()
    }

    if command == "-w4" {
        writev4();
    }

    if command == "-w6" {
        writev6()
    }
}