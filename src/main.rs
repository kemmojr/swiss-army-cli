fn main() {
    println!("Welcome to my swiss army CLI utility.");
    println!("Use --help for a list of all currently implemented commands.");

    let supported_commands: [&str; 1] = ["ls"];
    for command in supported_commands {
        let output: &str = match command {
            "ls" => "unfortunately I have just gotten started on this command",
            _ => "",
        };

        println!("{}", output)
    }
}
