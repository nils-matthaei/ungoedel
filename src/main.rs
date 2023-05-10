use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let gödel = args[1].clone();
    ungödel(gödel);
}

fn ungödel(gödel: String) {
    let mut valid = true;
    if !gödel.starts_with("1") || !gödel.ends_with("0") {
        valid = false;
    }
    let mut turing_machine: Vec<Transition> = Vec::new();
    let mut mode = Mode::State;

    let mut state = 0;
    let mut symbol = 0;
    let mut direction = 0;
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
                } else if character == '0' {
                    turing_machine.push(build_transition(state, symbol + 1, direction + 1));
                    state = 0;
                    symbol = 0;
                    direction = 0;
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

enum Mode {
    State,
    Symbol,
    Direction,
}

struct Transition {
    defined: bool,
    state: u32,
    symbol: char,
    direction: char,
}

impl Transition {
    fn to_string(&self) -> String {
        let ret: String;
        if self.defined {
            ret = format!("(Z{}, {}, {})", self.state, self.symbol, self.direction);
        } else {
            ret = String::from("(   --   )");
        }
        return ret;
    }
}

fn build_transition(state: u32, symbol: u32, direction: u32) -> Transition {
    let mut defined: bool = true;
    let symbol_char: char;
    match symbol {
        1 => {
            symbol_char = '0';
        }
        2 => {
            symbol_char = '1';
        }
        3 => {
            symbol_char = '□';
        }
        4 => symbol_char = '-',
        _other => symbol_char = '?',
    }
    let direction_char: char;
    match direction {
        1 => direction_char = 'L',
        2 => direction_char = 'N',
        3 => direction_char = 'R',
        _other => direction_char = '?',
    }

    if state == 1 && symbol == 4 && direction == 1 {
        defined = false;
    }

    Transition {
        defined: (defined),
        state: (state - 1),
        symbol: (symbol_char),
        direction: (direction_char),
    }
}
