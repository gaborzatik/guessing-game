use super::control_flow::InputType;

fn separator() {
    eprintln!("--------------------");
}

pub fn new_game() {
    separator();
    eprintln!("Welcome to the Guessing Game!");
    eprintln!("You have to find the secret number!");
    eprintln!("The number is located between 0 and 100");
    eprintln!();
    eprintln!("If you want to quit press 'q' or 'Q'");
}

pub fn end_game() {
    eprintln!("Thanks for the game, Good bye!");
    separator();
}

pub fn ask_user_input(selector: &InputType) {
    separator();
    match selector {
        InputType::AskForNumber => eprintln!("Guess a number"),
        InputType::AskIfQuit => {
            eprintln!("Would you like to start a new game?");
            eprintln!("Press 'Y' or 'y' if you want to continue.");
            eprintln!("Press 'N' or 'n' if you want to continue.");
        }
    }
}

pub fn invalid_input_for_limits() {
    eprintln!("The input is not in between the limits.");
    eprintln!("Try again.");
}

pub fn input_smaller() {
    eprintln!("Guess is less than the target.");
}

pub fn input_equal() {
    eprintln!("Guess is correct. You won!");
}

pub fn input_greater() {
    eprintln!("Guess is greater than the target.");
}
