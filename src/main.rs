use std::io::Write;

fn user_input() {

    print!("finsight >^º> ");

    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let _input = std::io::stdin();

    match _input.read_line(&mut buffer) {
        Ok(_) => {
            println!("You typed: {}", buffer.trim());
        },
        Err(error) => {
            println!("Error getting standard input: {}", error);
        }
    }
}

fn main() {
    loop {
        user_input();
   }
}

