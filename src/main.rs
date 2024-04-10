extern crate chrono;
extern crate serde;

use std::io::Write;
use std::process;
use std::string::String;
use std::fs;

use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
struct Session {
    expenses: Vec<Expense>,
    incomes: Vec<Income>,
}

#[derive(Serialize, Deserialize)]
struct Expense {
    id: u32,
    amount: f64,
    date: NaiveDate,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Income {
    id: u32,
    amount: f64,
    date: NaiveDate,
    description: String,
}

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

fn load_session_data(file_path: &str) -> Result<Session, serde_json::Error>
{
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    serde_json::from_str(&data)
}

fn _save_session_data(file_path: &str, session: &Session)
{
    let data = serde_json::to_string(session).expect("Failed to serialize session");
    fs::write(file_path, data).expect("Failed to write file");
}

fn create_session_data(file_path: &str) -> Result<Session, serde_json::Error>
{
    let session = Session {
        expenses: Vec::new(),
        incomes: Vec::new(),
    };

    let data = serde_json::to_string(&session)?;

    fs::write(file_path, data).expect("Failed to write file");

    Ok(session)
}

fn create_new_session() -> Option<String>
{
    FileDialog::new()
        .set_title("Save new session file")
        .save_file()
        .map(|path| path.display().to_string())
}

fn user_login() -> Result<Session, Box<dyn std::error::Error>>
{
    println!("Do you want to [L]oad or [C]reate a new session?");
    let mut command = String::new();
    let _input = std::io::stdin().read_line(&mut command).expect("Failed to read line");
    match command.to_uppercase().trim()
        {
            "L" => {
                let loaded_path: String = load_session().expect("Not a valid path");
                let session = load_session_data(&loaded_path);
                Ok(session?)
            },
            "C" => {
                // User selects directory
                let loaded_path: String = create_new_session().expect("Not a valid path");
                let session = create_session_data(&loaded_path);
                Ok(session?)
            },
            _ => Err("Invalid option.".into()),
        }
}

fn main()
{
    let _session = user_login();

    loop
    {
        user_input();
    }
}

