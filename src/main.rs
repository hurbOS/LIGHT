extern crate colorful;
use std::env;
mod commands;

fn command_parse() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match &command[..] {
        "-h" | "--help" => {
            commands::help();
        },
        "-i" | "--install" => {
            commands::install();
        }
        "-u" | "--update" => {
            commands::update();
        }
        _ => {
            print!("Invalid command, use --help\n");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            command_parse();
        },
        _ => {
            commands::help();
        }
    }
}
