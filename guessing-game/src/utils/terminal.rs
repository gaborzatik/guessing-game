use super::control_flow::InputType;
use super::log;
use std::io::{self, Write};

fn read_stdin() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn flush_stdout() -> io::Result<()> {
    io::stdout().flush()
}

pub fn read(selector: &InputType) -> Result<String, std::io::Error> {
    flush_stdout()?;
    log::ask_user_input(selector);
    read_stdin()
}
