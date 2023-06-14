use super::transition::*;

pub fn pretty_printer(_turing_machine: Vec<Transition>) {
    let mut turing_machine = _turing_machine;
    let offset: usize = (turing_machine.len() % 3) + 1;
    for _ in 0..offset {
        turing_machine.push(build_transition(1, 4, 1));
    }

    let mut i: u8 = 0;
    let mut state = 0;
    let max_states = turing_machine.len() / 3;
    println!("   |     0      |     1      |     □      |");
    println!("---|------------|------------|------------|");
    print!("Z0 | ");
    for transition in turing_machine {
        print!("{} | ", transition.to_string());
        i = (i + 1) % 3;
        if i == 0 {
            println!("");
            state += 1;
            if state < max_states {
                print!("Z{state} | ");
            }
        }
    }
}
