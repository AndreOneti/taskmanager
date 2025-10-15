use colored::Colorize;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{Result, Write, stdout};

pub fn read_string(prompt: &str) -> String {
    print!("{}: ", prompt.to_string().green());
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().to_string(),
        Err(_) => {
            println!("Erro de leitura");
            std::process::exit(1);
        }
    }
}

pub fn clear_terminal() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    Ok(())
}
