// Importing moduls
mod utils;

use utils::control_flow;
use utils::control_flow::InputType;
use utils::log;
use utils::random_number;
use utils::terminal;
use utils::util;

fn main() {
    utils::log::new_game();

    'new_target: loop {
        let target = random_number::new();
        '_new_input: loop {
            // It flushes the std output and reads the stdin.
            // In case of any io::std error, it returns
            let from_terminal = match terminal::read(&InputType::AskForNumber) {
                Ok(input) => input,
                Err(error) => {
                    eprintln!("Failed to read input: {error:?}.");
                    continue;
                }
            };

            // If "q" or "Q" is the input as string, the game end itself.
            if control_flow::if_quit(&from_terminal) {
                break 'new_target;
            }

            let num = match util::string_to_i32(&from_terminal) {
                Ok(number) => number,
                Err(error) => {
                    eprintln!("Failed to convert the input to i32: {error:?}.");
                    continue;
                }
            };

            // The input should in between of 0 and 100. Otherwise, it gives an error log and not compare.
            if !util::check_limits(num) {
                continue;
            }

            if !util::compare_input_with_target(num, target) {
                continue;
            }

            '_if_quit: loop{
                let from_terminal = match terminal::read(&InputType::AskIfQuit) {
                    Ok(input) => input,
                    Err(error) => {
                        eprintln!("Failed to read input: {error:?}.");
                        continue;
                    }
                };
    
                match control_flow::if_new_game(&from_terminal) {
                    control_flow::IfNewGame::Yes => {
                        break '_new_input;
                    }
                    control_flow::IfNewGame::No => {
                        log::end_game();
                        break 'new_target;
                    }
                    control_flow::IfNewGame::Invalid => {
                        eprintln!("Invalid input, try again with 'Y' or 'N'.");
                    }
                }
            }
            
        }
    }
}
