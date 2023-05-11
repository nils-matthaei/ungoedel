pub struct Transition {
    defined: bool,
    state: u32,
    symbol: char,
    direction: char,
}

impl Transition {
    pub fn to_string(&self) -> String {
        let ret: String;
        if self.defined {
            ret = format!("(Z{}, {}, {})", self.state, self.symbol, self.direction);
        } else {
            ret = String::from("(   --   )");
        }
        return ret;
    }
}

pub fn build_transition(state: u32, symbol: u32, direction: u32) -> Transition {
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