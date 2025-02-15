use std::fmt::{self, Display};
use std::process::exit;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct VecString {
    vec: Vec<String>,
}

impl FromStr for VecString {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().split_whitespace().count() > 0 {
            return Ok(VecString {
                vec: s.trim().split_whitespace().map(|s| s.to_string()).collect(),
            });
        }

        return Ok(VecString { vec: vec![] });
    }
}

impl Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for s in &self.vec {
            output.push_str(&s);
            output.push_str(" ");
        }

        return write!(f, "{}", output);
    }
}

fn main() -> ! {
    println!("Welcome to my swiss army CLI utility.");
    println!("run 'swiss --help' for a list of all currently implemented commands.");

    loop {
        let mut input = std::io::stdin().read_line(&mut String::new());

        input = match input {
            Ok(_) => input,
            Err(_) => {
                println!("There was an error reading the input. Please try again.");
                continue;
            }
        };

        let vec_string_input: VecString = input.unwrap().to_string().parse().unwrap();

        println!("{}", vec_string_input);

        let args = vec_string_input.vec;
        let args_len = args.len();

        if args.len() > 1 {
            println!("The command that will be executed is '{}'", args[0]);
        }

        if args.len() > 2 {
            print!("The command and args are '{}'", args[0].to_string());
            print!("{}", args[1])
        }

        if args.len() > 3 {
            print!("The command and args are '{}'", args[0]);
            print!("{}", args[1]);
            print!("{}", args[2]);
        }

        let output: &str = match &args[0] {
            c if c == "swiss" && args_len > 2 && (args[1] == "--help" || args[1] == "help") => {
                "Commands:
                - 'ls' - list all files in the current directory
                - 'quit' or 'exit' - exit the program
                - 'help' - display this help message"
            }
            c if c == "ls" => "unfortunately I have just gotten started on this command and it isn't fully functional yet",
            c if c == "quit" || c == "exit" => {exit(0)},
            c if c.is_empty() => "Please enter a command. To see a list of all supported commands, run 'swiss --help'",
            _ => "This command is not yet supported. To see a list of all supported commands, run 'swiss --help'",
        };

        println!("{}", output);
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
