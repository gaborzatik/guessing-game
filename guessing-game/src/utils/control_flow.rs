use super::log;
pub enum IfNewGame {
    Yes,
    No,
    Invalid,
}

pub enum InputType {
    AskForNumber,
    AskIfQuit,
}

pub fn if_quit(input: &str) -> bool {
    if (input == "q") || (input == "Q") {
        log::end_game();
        return true;
    }
    false
}

pub fn if_new_game(input: &str) -> IfNewGame {
    match input {
        "Y" | "y" => IfNewGame::Yes,
        "N" | "n" => IfNewGame::No,
        _ => IfNewGame::Invalid,
    }
}
