use std::io;
use std::io::Write;

pub fn handle_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Falha ao limpar stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    input.trim().to_string()
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}