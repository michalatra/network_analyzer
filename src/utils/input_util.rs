use std::io::Write;

pub fn read_input<T>(prompt: &str) -> Option<T> where T: std::str::FromStr {
    let mut input = String::new();
    print!("| {}> ", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse() {
        Ok(value) => Some(value),
        Err(_) => None
    }
}

pub fn read_command(prompt: &str) -> String {
    let mut command = String::new();
    print!("[{}] > ", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut command).expect("Failed to parse command");
    command
}