use std::io::Write;
use std::process;
use std::string::String;
use rfd::FileDialog;

fn user_input()
{

    print!("{}", "finsight ");
    std::io::stdout().flush().unwrap();
    let mut command = String::new();
    let _input = std::io::stdin().read_line(&mut command).expect("Failed to read line");

    let amount_str = "100";

    match command.trim()
    {
        "exit" => {
            println!("exiting...");
            process::exit(0);
        },
        "help" => {
            // read from a text file.
            const HELP_MESSAGE: &str = include_str!("resources/help_message.txt");
            println!("{}", HELP_MESSAGE);
        },
        "ADD EXPENSE" => {
            let _amount = amount_str.parse::<f64>().expect("Invalid amount");
        },
        _ => {
            println!("Unknown command. Type HELP for a list of commands.");
        }
    }
}

fn load_session() -> Option<String>
{
    FileDialog::new()
        .set_title("Select a session file")
        .pick_file()
        .map(|path| path.display().to_string())
}

fn user_login()
{
    println!("Do you want to [L]oad or [C]reate a new session?");

    let mut command = String::new();
    let _input = std::io::stdin().read_line(&mut command).expect("Failed to read line");

match command.to_uppercase().trim()
    {
        "L" => {
            load_session();
        },
        "C" => {
            println!("Enter new session name: ");
            let mut session_name = String::new();
            let _input = std::io::stdin().read_line(&mut session_name).expect("Failed to read line");
            println!("You chose the session {}", session_name.trim());
        },
        _ => println!("Invalid option."),
    }
}

fn main()
{
    user_login();

    loop
    {
        user_input();
    }
}

