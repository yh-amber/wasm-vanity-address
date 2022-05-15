extern crate wasm_vanity_address;

use wasm_vanity_address::SolKeypair;

fn main() {
    let solanity = SolKeypair::generate("ERT");
    println!("Result:: {:#?}", solanity);
}