use super::transition;
enum Mode {
    State,
    Symbol,
    Direction,
}

pub fn ungödel(gödel: String) {
    let mut valid = true;
    if !gödel.starts_with('1') || !gödel.ends_with('0') {
        valid = false;
    }
    let mut turing_machine: Vec<transition::Transition> = Vec::new();
    let mut mode = Mode::State;

    let mut state = 0;
    let mut symbol = 1;
    let mut direction = 1;
    for character in gödel.chars() {
        match mode {
            Mode::State => {
                if character == '1' {
                    state += 1;
                } else if character == '0' {
                    mode = Mode::Symbol;
                } else {
                    valid = false;
                    break;
                }
            }
            Mode::Symbol => {
                if character == '0' {
                    symbol += 1;
                    if symbol > 4 {
                        valid = false;
                        break;
                    }
                } else if character == '1' {
                    mode = Mode::Direction;
                } else {
                    valid = false;
                    break;
                }
            }
            Mode::Direction => {
                if character == '1' {
                    direction += 1;
                    if direction > 3{
                        valid = false;
                        break;
                    }
                } else if character == '0' {
                    turing_machine.push(transition::build_transition(state, symbol, direction));
                    state = 0;
                    symbol = 1;
                    direction = 1;
                    mode = Mode::State;
                } else {
                    valid = false;
                    break;
                }
            }
        }
    }
    if valid {
        for transition in turing_machine {
            println!("{}", transition.to_string());
        }
    } else {
        println!("Invalid")
    }
}