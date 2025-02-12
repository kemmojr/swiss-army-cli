use std::process::exit;
use std::{env, usize};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Welcome to my swiss army CLI utility.");
    println!("run 'swiss --help' for a list of all currently implemented commands.");

    let mut running = true;

    while running {
        let args: Vec<String> = env::args().collect();

        let args_len = args.len();
        match args_len {
            x if x > 1 => println!("The command that will be executed is '{}'", args[1]),
            x if x > 0 => {
                println!(
                    "Please pass in the command you wish to run after the 'cargo run' command"
                );
                continue;
            }
            _ => {
                println!("No args provided");
                continue;
            }
        }

        let output: &str = match &args[1] {
            c if c == "ls" => "unfortunately I have just gotten started on this command and it isn't fully functional yet",
            c if c == "quit" || c== "exit" => {exit(0)},
            _ => "This command is not yet supported. To view a list of commands, 'cargo run' with --help",
        };

        println!("{}", output)
    }

    /*
       // Create a path to the desired file
           let path = Path::new("hello.txt");
           let display = path.display();

           // Open the path in read-only mode, returns `io::Result<File>`
           let mut file = match File::open(&path) {
               Err(why) => panic!("couldn't open {}: {}", display, why),
               Ok(file) => file,
           };

           // Read the file contents into a string, returns `io::Result<usize>`
           let mut s = String::new();
           match file.read_to_string(&mut s) {
               Err(why) => panic!("couldn't read {}: {}", display, why),
               Ok(_) => print!("{} contains:\n{}", display, s),
           }

           // `file` goes out of scope, and the "hello.txt" file gets closed
    */
}
