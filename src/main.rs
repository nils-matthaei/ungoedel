use std::env;
use ungoedel::goedel::ungödel;
fn main() {
    let args: Vec<String> = env::args().collect();
    let gödel = args[1].clone();
    ungödel(gödel);
}