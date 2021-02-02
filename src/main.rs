use std::env;

fn help() {
    print!("LIGHT Help\n\n\nCommands:\n\nInstall: --install/-i [package]\nSearch, download, and install a package from enabled repositories.")
}

fn command_parse() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "-i" || command == "--install" {
        print!("Test")
    } else {
        print!("Invalid command")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            command_parse();
        },
        _ => {
            help();
        }
    }
}
