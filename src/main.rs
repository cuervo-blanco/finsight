
fn user_input() {

    let mut buffer = String::new();
    let _input = std::io::stdin();
    println!("finsight >^º> ");

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

