use std::io::Write;
use std::process;

fn user_input()
{

    print!("finsight >^º> ");

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
                    println!("Exiting");
                    process::exit(0);
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

