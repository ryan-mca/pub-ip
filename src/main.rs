use std::fs::File;
use std::io::{stdout, Write};
use curl::easy::Easy;
use std::env
use std::process::exit;

fn write() {
    let mut file = File::create(pub-ip.txt)
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