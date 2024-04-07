use std::io::Write;
use std::process;

fn user_input()
{

    // if there is a menu item selected (command) append it to the last_command
    // else
    print!("{}", "finsight ");

    std::io::stdout().flush().unwrap();

    let mut command = String::new();
    let _input = std::io::stdin();

    match _input.read_line(&mut command)
    {
        Ok(_) =>
        {
            match command.trim()
            {
                "exit" => {
                    println!("exiting...");
                    process::exit(0);
                },
                "help" | "HELP" => {
                    // read from a text file.
                    const HELP_MESSAGE: &str = include_str!("resources/help_message.txt");
                    println!("{}", HELP_MESSAGE);
                },
                _ => {
                    println!("Unknown command");
                }
            }
        },
        Err(error) =>
        {
            println!("Error getting standard input: {}", error);
        }
    }
}

fn main()
{
    loop
    {
        user_input();
    }
}

